use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;

use main::{input_parsing::input_parsing, ThreadPool};

const HTTP_RESPONSE_LINK_OK: &str = "HTTP/1.1 200 OK";
const HTTP_RESPONSE_LINK_BAD_REQUEST: &str = "HTTP/1.1 400 BAD REQUEST";

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let thread_pool = ThreadPool::new(8);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread_pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down gracefully...");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 256];
    stream.read(&mut buffer).unwrap();

    let read_buffer = from_utf8(&buffer).unwrap();
    let request_status_line = read_buffer.split("\r\n").next();

    let route = match request_status_line {
        Some(line) => {
            let parts = line.split(" ").collect::<Vec<&str>>();
            match parts[..] {
                ["GET", route, _] => Ok(route),
                _ => Err("Invalid request"),
            }
        }
        None => Err("Invalid request"),
    };

    println!("{:?}", request_status_line);

    let response = match route {
        Ok(route) => handle_request(route),
        Err(message) => String::from(message),
    };

    close_stream(stream, response);
}

fn handle_request(route: &str) -> String {
    match parse_route(route) {
        Ok(ParsedRoute { number1, number2 }) => {
            let sum = input_parsing::Number::add(number1, number2);

            println!("{:?}+{:?}={:?}", number1, number2, sum);

            let contents = fs::read_to_string("hello.html")
                .unwrap()
                .replace("{{NUMBER_1}}", &number1.to_string())
                .replace("{{NUMBER_2}}", &number2.to_string())
                .replace("{{SUM}}", &sum.to_string());

            format!(
                "{}\r\nContent-Length: {}\r\n\r\n{}",
                HTTP_RESPONSE_LINK_OK,
                contents.len(),
                contents
            )
        }
        Err(message) => {
            format!(
                "{}\r\nContent-Length: {}\r\n\r\n{}",
                HTTP_RESPONSE_LINK_BAD_REQUEST,
                message.len(),
                message
            )
        }
    }
}

fn parse_route(route: &str) -> Result<ParsedRoute, String> {
    let parts = route.split("/").collect::<Vec<&str>>();
    match parts[..] {
        [_, "add", number1, number2] => ParsedRoute::create(number1, number2),
        _ => Err(String::from("Inputs must be numbers")),
    }
}

struct ParsedRoute {
    number1: input_parsing::Number,
    number2: input_parsing::Number,
}

impl ParsedRoute {
    fn create(str1: &str, str2: &str) -> Result<ParsedRoute, String> {
        let parsed_str1 = input_parsing::parse_string_to_number(str1);
        let parsed_str2 = input_parsing::parse_string_to_number(str2);

        match (parsed_str1, parsed_str2) {
            (Ok(number1), Ok(number2)) => Ok(ParsedRoute { number1, number2 }),
            _ => Err(String::from("Inputs must be numbers")),
        }
    }
}

fn close_stream(mut stream: TcpStream, response: String) {
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
