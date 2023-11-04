#[macro_use]
extern crate prettytable;

mod file_io;
mod flapjack_stack;
mod option_repl;
mod updates;

use flapjack_stack::flapjack_stack_builder::FlapJackStackBuilder;
use option_repl::OptionRepl;
use std::process::exit;

// TODO: show last comment on the table (maybe)
// or add a way to check comments without using the log
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // We check to make sure that there is not a new update available.
    // This block of logic exits with an error if there is an error,
    // or exits the program if the user decides to update.
    if updates::try_update()? {
        eprintln!("Binary updated. Please restart the program!");
        exit(0)
    }

    let path = file_io::init_log_db()?;

    let stack = FlapJackStackBuilder::from_file(&path).build();
    let repl = OptionRepl::new(stack);
    repl.start();

    Ok(())
}
