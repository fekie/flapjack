mod file_io;
mod flapjack_stack;

use clap::Parser;
use flapjack_stack::flapjack_stack_builder::FlapJackStackBuilder;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    name: String,

    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let path = file_io::init_log_db();
    let content = file_io::read_raw_db_contents(&path).unwrap();

    let stack = FlapJackStackBuilder::new(&content);
    // todo write cli interface
}
