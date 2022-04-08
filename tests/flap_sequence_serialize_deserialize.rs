use flapjack::flap_sequence::{Comment, Directive, Flap, FlapSequenceBuilder};

// TODO: write function for serialization

#[test]
fn flap_sequence_from_file() {
    let mut builder =
        FlapSequenceBuilder::from_file("example_logs/serialize_deserialize/input.flap");
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
