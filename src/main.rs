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
    match updates::try_update() {
        Ok(success) => {
            if success {
                eprintln!("Binary updated. Please restart the program!");
                exit(0);
            }
        }
        Err(_) => {
            // We go ahead and let them continue. This can happen
            // if a release does not have an update available.
            eprintln!("Error trying to update. A precompiled version may not be available. Please update manually.")
        }
    }

    let path = file_io::init_log_db()?;

    let stack = FlapJackStackBuilder::from_file(&path).build();
    let repl = OptionRepl::new(stack);
    repl.start();

    Ok(())
}
