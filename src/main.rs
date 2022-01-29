mod logic;
mod ui;

fn main() {
    if let Err(error) = ui::main_loop() {
        println!("Something bad happened: {error:?}");
    }
}
