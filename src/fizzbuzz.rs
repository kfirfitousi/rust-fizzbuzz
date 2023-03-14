use colored::Colorize;
use std::fmt::{Display, Formatter, Result};

pub enum FizzBuzz {
    Fizz,
    Buzz,
    FizzBuzz,
    Number(u32),
}

impl FizzBuzz {
    fn new(number: u32) -> Self {
        match (number % 5 == 0, number % 3 == 0) {
            (true, true) => FizzBuzz::FizzBuzz,
            (false, true) => FizzBuzz::Fizz,
            (true, false) => FizzBuzz::Buzz,
            (false, false) => FizzBuzz::Number(number),
        }
    }
}

impl Display for FizzBuzz {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            FizzBuzz::Fizz => write!(f, "{}", "Fizz".blue()),
            FizzBuzz::Buzz => write!(f, "{}", "Buzz".yellow()),
            FizzBuzz::FizzBuzz => write!(f, "{}{}", "Fizz".blue(), "Buzz".yellow()),
            FizzBuzz::Number(number) => write!(f, "{}", number),
        }
    }
}

pub fn fizzbuzz(number: u32) {
    (1..=number).for_each(|x| println!("{}", FizzBuzz::new(x)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(FizzBuzz::new(1).to_string(), "1");
        assert_eq!(FizzBuzz::new(2).to_string(), "2");
        assert_eq!(FizzBuzz::new(3).to_string(), "Fizz".blue().to_string());
        assert_eq!(FizzBuzz::new(4).to_string(), "4");
        assert_eq!(FizzBuzz::new(5).to_string(), "Buzz".yellow().to_string());
        assert_eq!(FizzBuzz::new(6).to_string(), "Fizz".blue().to_string());
        assert_eq!(FizzBuzz::new(7).to_string(), "7");
        assert_eq!(FizzBuzz::new(8).to_string(), "8");
        assert_eq!(FizzBuzz::new(9).to_string(), "Fizz".blue().to_string());
        assert_eq!(FizzBuzz::new(10).to_string(), "Buzz".yellow().to_string());
        assert_eq!(FizzBuzz::new(11).to_string(), "11");
        assert_eq!(FizzBuzz::new(12).to_string(), "Fizz".blue().to_string());
        assert_eq!(FizzBuzz::new(13).to_string(), "13");
        assert_eq!(FizzBuzz::new(14).to_string(), "14");
        assert_eq!(
            FizzBuzz::new(15).to_string(),
            "Fizz".blue().to_string() + "Buzz".yellow().to_string().as_str()
        );
        assert_eq!(FizzBuzz::new(2048).to_string(), "2048");
        assert_eq!(
            FizzBuzz::new(3000).to_string(),
            "Fizz".blue().to_string() + "Buzz".yellow().to_string().as_str()
        );
        assert_eq!(FizzBuzz::new(5555).to_string(), "Buzz".yellow().to_string());
        assert_eq!(FizzBuzz::new(9999).to_string(), "Fizz".blue().to_string());
    }
}
