use flapjack;
use flapjack::flapjack_stack;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // we do this instead of loading a file
    let example_log = read_from_example_file("example_logs/directives/input.flap");
    let mut foo = flapjack_stack::FlapSequenceBuilder::new(example_log);
    println!("{:?}", foo);
    let bar = foo.build();
    println!("{:?}", bar);
    bar.serialize_to_file("meow.flap");
    let temp_directory = env::temp_dir();
    println!("{:?}", temp_directory);
}

fn read_from_example_file(path: &str) -> String {
    let file = fs::read_to_string(path);
    match file {
        Ok(contents) => contents,
        Err(_) => panic!("Cant find this file!"),
    }
}
