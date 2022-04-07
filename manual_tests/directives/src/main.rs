use flapjack;
use flapjack::flap_sequence;
use std::fs;

fn main() {
    // we do this instead of loading a file
    let example_log = read_from_example_file("example_logs/directives/input.flap");
    let foo = flap_sequence::DirectiveSequenceBuilder::new(example_log).build();
    println!("{:?}", foo);
}

fn read_from_example_file(path: &str) -> String {
    let file = fs::read_to_string(path);
    match file {
        Ok(contents) => contents,
        Err(_) => panic!("Cant find this file!"),
    }
}
