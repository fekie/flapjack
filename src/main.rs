#[macro_use]
extern crate prettytable;
mod file_io;
mod flapjack_stack;
mod option_repl;

use flapjack_stack::flapjack_stack_builder::FlapJackStackBuilder;
use option_repl::OptionRepl;

// TODO: create cases for when no wallets exist
fn main() {
    let path = file_io::init_log_db();

    let stack = FlapJackStackBuilder::from_file(&path).build();
    let repl = OptionRepl::new(stack);
    repl.start();
}
