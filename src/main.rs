mod file_io;
mod flapjack_stack;
mod option_repl;

use flapjack_stack::flapjack_stack_builder::FlapJackStackBuilder;
use option_repl::OptionRepl;

fn main() {
    let path = file_io::init_log_db();
    let content = file_io::read_raw_db_contents(&path).unwrap();

    let stack = FlapJackStackBuilder::new(&content).build();
    let repl = OptionRepl::new(stack);
    repl.start();
}
