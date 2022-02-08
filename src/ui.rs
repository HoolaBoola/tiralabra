use crate::logic::Calculator;
use std::io::{stdin, stdout, Write};

pub fn main_loop() -> Result<(), Box<dyn std::error::Error>> {
    let mut calculator = Calculator::new();

    let control_key = "?";

    let mut input = String::new();
    loop {
        print!("> ");

        // "> " won't appear without flushing
        stdout().flush()?;
        stdin().read_line(&mut input)?;

        if input.starts_with(control_key) {
            break;
        }

        match calculator.calculate_infix(&input) {
            Ok(result) => println!("{result}"),
            Err(err) => eprintln!("Error:\n{err}"),
        }

        input.clear();
    }
    Ok(())
}
