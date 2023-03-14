mod fizzbuzz;

use fizzbuzz::fizzbuzz;

fn main() {
    println!("FizzBuzz");

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
