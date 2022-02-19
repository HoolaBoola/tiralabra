use crate::logic::Calculator;
use rustyline::error::ReadlineError;
use rustyline::Editor;

/// The main REPL for the calculator. 
///
pub fn main_loop() -> Result<(), std::io::Error> {
    let mut calculator = Calculator::new();
    
    let mut rl = Editor::<()>::new();

    let control_key = "?";
    println!("To exit, enter {control_key}quit");

    loop {
        let readline = rl.readline(">> ");
        let input = match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                line
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C, quitting...");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D, quitting...");
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
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
