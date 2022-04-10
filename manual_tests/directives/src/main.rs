use flapjack;
use flapjack::flapjack_stack;
use std::fs;

fn main() {
    // we do this instead of loading a file
    let example_log = read_from_example_file("example_logs/directives/input.flap");
    let mut foo = flapjack_stack::flap_sequence_builder::FlapSequenceBuilder::new(&example_log);
    let bar = foo.build();
    bar.serialize_to_file("example_logs/directives/output.flap");
    println!("{:?}", bar.db);
}

fn read_from_example_file(path: &str) -> String {
    let file = fs::read_to_string(path);
    match file {
        Ok(contents) => contents,
        Err(_) => panic!("Cant find this file!"),
    }
}
