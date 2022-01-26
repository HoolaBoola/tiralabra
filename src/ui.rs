use crate::logic::calculator::calculate_infix;
use std::io::{stdin, stdout, Write};

pub fn main_loop() -> Result<(), Box<dyn std::error::Error>> {
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

        println!("{}", calculate_infix(&input));
        input.clear();
    }
    Ok(())
}
