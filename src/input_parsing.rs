pub mod input_parsing {
    pub fn parse_string_to_number(input: &str) -> Result<Number, String> {
        match input.parse::<f64>() {
            Ok(value) => Ok(Number::Float(value)),
            Err(_) => match input.parse::<i32>() {
                Ok(value) => Ok(Number::Integer(value)),
                Err(_) => Err(format!("Could not parse {} into number.", input)),
            },
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub enum Number {
        Integer(i32),
        Float(f64),
    }

    impl Number {
        pub fn add(number1: Number, number2: Number) -> Number {
            match (number1, number2) {
                (Number::Integer(v1), Number::Integer(v2)) => Number::Integer(v1 + v2),
                (Number::Integer(v1), Number::Float(v2)) => Number::Float(v1 as f64 + v2),
                (Number::Float(v1), Number::Integer(v2)) => Number::Float(v1 + v2 as f64),
                (Number::Float(v1), Number::Float(v2)) => Number::Float(v1 + v2),
            }
        }

        pub fn to_string(self) -> String {
            match self {
                Number::Integer(value) => value.to_string(),
                Number::Float(value) => value.to_string(),
            }
        }
    }
}
