use colored::Colorize;
use std::fmt::{Display, Formatter, Result};

enum FizzBuzz {
    Fizz,
    Buzz,
    FizzBuzz,
    Number(u32),
}

fn fizzbuzz(number: u32) {
    (1..=number).for_each(|x| println!("{}", <u32 as Into<FizzBuzz>>::into(x)))
}

fn main() {
    println!("Unnecessarily complex FizzBuzz");

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

        fizzbuzz(number);
        return;
    }
}

// number --> FizzBuzz
impl Into<FizzBuzz> for u32 {
    fn into(self) -> FizzBuzz {
        match (self % 5, self % 3) {
            (0, 0) => FizzBuzz::FizzBuzz,
            (_, 0) => FizzBuzz::Fizz,
            (0, _) => FizzBuzz::Buzz,
            (_, _) => FizzBuzz::Number(self),
        }
    }
}

// FizzBuzz --> formatted string
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
