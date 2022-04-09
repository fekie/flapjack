use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

use self::flapjack::{Command, FlapJack};

pub mod flap_sequence_builder;
pub mod flapjack;

const CREATION_TYPES: [&str; 1] = ["account"];

/// A sequence of `Flap`s that each contain either a `Directive` or a `Comment`.
/// Each flap in the sequence retains its order.
#[warn(missing_docs)]
#[derive(Debug)]
pub struct FlapJackStack {
    pub flaps: Vec<FlapJack>,
    db: FlapJackDb,
}

impl FlapJackStack {
    pub fn new(flaps: Vec<FlapJack>) -> Self {
        let db = FlapJackDb::from_flaps(&flaps);
        Self { flaps, db }
    }

    pub fn serialize_to_file(&self, path: &str) {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(path)
            .unwrap();

        let serialized = self.serialize();
        file.write(serialized.as_bytes()).unwrap();
    }

    pub fn serialize(&self) -> String {
        let mut serialized = String::new();
        for (i, flap) in self.flaps.iter().enumerate() {
            serialized.push_str(&flap.serialize());

            // add a new line if it is not the last line
            if (i + 1) != self.flaps.len() {
                serialized.push_str("\n");
            }
        }
        serialized
    }
}

#[derive(Debug)]
pub struct FlapJackDb {
    // if the transaction does not exist, the program will panic
    available_transaction_types: Vec<String>,
    // each type of transaction will have a vector of transactions in order
    transactions: HashMap<String, Vec<TransactionInfo>>,
}

impl FlapJackDb {
    pub fn new() -> Self {
        Self {
            available_transaction_types: Vec::new(),
            transactions: HashMap::new(),
        }
    }

    pub fn from_flaps(flaps: &Vec<FlapJack>) -> Self {
        let db = Self {
            available_transaction_types: Vec::new(),
            transactions: HashMap::new(),
        };

        // TODO: make this work
        for flapjack in flaps {
            match flapjack {
                FlapJack::Comment(comment) => {
                    // do nothing
                }
                FlapJack::Directive(directive) => {
                    let command = &directive.command;
                    let params = directive.params.as_slice();

                    match command {
                        Command::CREATE => {
                            let create_type = params
                                .get(0)
                                .expect("Creation type argument was not found!");

                            let mut found = false;
                            for ctype in CREATION_TYPES {
                                if ctype == create_type {
                                    found = true;
                                    break;
                                }
                            }

                            if !found {
                                panic!("An incorrect creation type was provided!")
                            }

                            // i stopped here
                            todo!()
                        }
                    }
                }
            }
        }

        db
    }
}

#[derive(Debug)]
pub struct TransactionInfo {}

#[cfg(test)]
mod tests {
    use crate::flapjack_stack::flap_sequence_builder::FlapSequenceBuilder;

    #[test]
    fn test_serialization() {
        let log = "# the program will register this line a comment\nCREATE account \"Checking (Bank)\"\nCREATE account \"Savings (Bank)\"";

        let seq = FlapSequenceBuilder::new(log.to_string()).build();
        let serialized = seq.serialize();

        assert_eq!(serialized, log);

        let seq_rebuilt = FlapSequenceBuilder::new(serialized.clone()).build();
        let serialized_again = seq_rebuilt.serialize();

        assert_eq!(serialized, serialized_again)
    }
}
