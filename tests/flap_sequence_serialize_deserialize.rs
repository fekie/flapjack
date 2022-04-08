use flapjack::flap_sequence::{Comment, Directive, Flap, FlapSequenceBuilder};
use std::env;

// do not write to the input path, only one test should write to the output path
const INPUT_FLAP_PATH: &str = "tests/test_files/serialize_deserialize/input.flap";

// TODO: write function for serialization

#[test]
fn flap_sequence_from_file() {
    let mut builder = FlapSequenceBuilder::from_file(INPUT_FLAP_PATH);
    let seq = builder.build();
    assert_eq!(
        seq.flaps[0],
        Flap::Comment(Comment::new(
            "# the program will register this line a comment".to_owned()
        ))
    );

    assert_eq!(
        seq.flaps[1],
        Flap::Directive(Directive {
            command: "CREATE".to_owned(),
            params: vec!["account".to_owned(), "Checking (Bank)".to_owned()]
        })
    );

    assert_eq!(
        seq.flaps[2],
        Flap::Directive(Directive {
            command: "CREATE".to_owned(),
            params: vec!["account".to_owned(), "Savings (Bank)".to_owned()]
        })
    );
}

fn flap_sequence_to_file() {
    let mut seq = FlapSequenceBuilder::from_file(INPUT_FLAP_PATH).build();
}
