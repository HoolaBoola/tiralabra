use crate::logic::Calculator;
use rustyline::error::ReadlineError;
use rustyline::Editor;

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
    
    let mut rl = Editor::<()>::new();

    println!("To exit, enter ?quit");
    let control_key = "?";

    loop {
        let readline = rl.readline(">> ");
        let input = match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                line
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C, quitting...");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D, quitting...");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        };

        if input.starts_with(control_key) {
            break;
        }

        match calculator.calculate_infix(input.trim()) {
            Ok(result) => println!(" {result}"),
            Err(err) => eprintln!("Error:\n{err}"),
        }

    }
    Ok(())
}
