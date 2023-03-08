use colored::Colorize;
use std::fmt::{Display, Formatter, Result};

enum FizzBuzz {
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

fn fizzbuzz(number: u32) {
    (1..=number).for_each(|x| println!("{}", FizzBuzz::new(x)))
}

fn main() {
    println!("Unnecessarily complex FizzBuzz");

    if let Ok(number) = std::env::args().nth(1).unwrap_or_default().parse() {
        return fizzbuzz(number);
    }

    loop {
        println!("Enter a number:");

        let mut number = String::new();

        std::io::stdin()
            .read_line(&mut number)
            .expect("Error reading line");
        print!("\x1b[1A\x1b[2K\x1b[1A\x1b[2K");

        let number: u32 = match number.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                print!("Not a number. ");
                continue;
            }
        };

        return fizzbuzz(number);
    }
}
