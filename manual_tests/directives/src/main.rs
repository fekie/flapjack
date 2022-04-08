use flapjack;
use flapjack::flap_sequence;
use std::fs;
use std::path::PathBuf;

fn main() {
    // we do this instead of loading a file
    let example_log = read_from_example_file("example_logs/directives/input.flap");
    let mut foo = flap_sequence::FlapSequenceBuilder::new(example_log);
    println!("{:?}", foo);
    let bar = foo.build();
    println!("{:?}", bar);
    bar.serialize_to_file("meow.flap");
}

fn read_from_example_file(path: &str) -> String {
    let file = fs::read_to_string(path);
    match file {
        Ok(contents) => contents,
        Err(_) => panic!("Cant find this file!"),
    }
}
