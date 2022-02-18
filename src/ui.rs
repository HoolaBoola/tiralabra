use crate::logic::Calculator;
use std::io::{stdin, stdout, Write};

/// The main REPL for the calculator. 
///
/// Input length in many terminal emulators is limited to 4096 characters, and anything
/// longer will be automatically trimmed.
///
/// See https://web.archive.org/web/20170315162529/http://blog.chaitanya.im/4096-limit
///
/// I'm unsure whether it's fixable easily.
///
pub fn main_loop() -> Result<(), std::io::Error> {
    let mut calculator = Calculator::new();
    
    println!("To exit, enter ?quit");
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

        match calculator.calculate_infix(input.trim()) {
            Ok(result) => println!("{result}"),
            Err(err) => eprintln!("Error:\n{err}"),
        }

        input.clear();
    }
    Ok(())
}
