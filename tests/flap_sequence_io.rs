use flapjack::flapjack_stack::flapjack::{Command, Comment, Directive, FlapJack};
use flapjack::flapjack_stack::flapjack_stack_builder::FlapJackStackBuilder;
use std::env;
use std::fs;

// do not write to the input path
const INPUT_FLAP_PATH: &str = "tests/test_files/serialize_deserialize/input.flap";

#[test]
fn flaps_from_file() {
    let mut builder = FlapJackStackBuilder::from_file(INPUT_FLAP_PATH);
    let seq = builder.build();

    assert_eq!(
        seq.flaps[0],
        FlapJack::Comment(Comment::new(
            "# the program will register this line a comment".to_owned()
        ))
    );

    assert_eq!(
        seq.flaps[1],
        FlapJack::Directive(Directive {
            command: Command::CREATE,
            params: vec!["Checking (Bank)".to_owned()]
        })
    );

    assert_eq!(
        seq.flaps[2],
        FlapJack::Directive(Directive {
            command: Command::CREATE,
            params: vec!["Savings (Bank)".to_owned()]
        })
    );
}

#[test]
fn flaps_to_file() {
    let seq = FlapJackStackBuilder::from_file(INPUT_FLAP_PATH).build();
    let serialized = seq.serialize();
    let temp_directory = env::temp_dir();
    let temp_path = temp_directory.join("example_log.flap");

    seq.serialize_to_file(&temp_path.to_string_lossy());
    let content = fs::read_to_string(&temp_path).unwrap();
    assert_eq!(content, serialized)
}
