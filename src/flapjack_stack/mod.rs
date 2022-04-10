use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

use self::flapjack::{Command, FlapJack};

pub mod flap_sequence_builder;
pub mod flapjack;

/// A sequence of `Flap`s that each contain either a `Directive` or a `Comment`.
/// Each flap in the sequence retains its order.
#[derive(Debug)]
pub struct FlapJackStack {
    pub flaps: Vec<FlapJack>,
    pub db: FlapJackDb,
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
    // each type of transaction will have a vector of transactions in order
    pub wallet_amounts: HashMap<String, f64>,
}

impl FlapJackDb {
    pub fn new() -> Self {
        Self {
            wallet_amounts: HashMap::new(),
        }
    }

    pub fn from_flaps(flaps: &Vec<FlapJack>) -> Self {
        let mut db = Self {
            wallet_amounts: HashMap::new(),
        };

        for flapjack in flaps {
            match flapjack {
                FlapJack::Comment(_comment) => {
                    // do nothing
                }
                FlapJack::Directive(directive) => {
                    let command = &directive.command;
                    let params = directive.params.as_slice();

                    match command {
                        Command::CREATE => {
                            let wallet_type =
                                params.get(0).expect("Wallet type argument was not found!");

                            db.wallet_amounts.insert(wallet_type.to_string(), 0.0);
                        }
                        Command::INCREMENT => {
                            let wallet_type =
                                params.get(0).expect("Wallet type argument was not found!");

                            let amount = params
                                .get(1)
                                .expect("Amount was not found")
                                .parse::<f64>()
                                .expect("Amount could not be parsed to a float.");

                            match db.wallet_amounts.get_mut(&wallet_type.to_string()) {
                                Some(wallet_balance) => {
                                    *wallet_balance += amount;
                                }
                                None => {
                                    panic!("Wallet type {} does not exist!", wallet_type)
                                }
                            };
                        }
                        Command::SET => {
                            let wallet_type =
                                params.get(0).expect("Wallet type argument was not found!");

                            let amount = params
                                .get(1)
                                .expect("Amount was not found")
                                .parse::<f64>()
                                .expect("Amount could not be parsed to a float.");

                            match db.wallet_amounts.get_mut(&wallet_type.to_string()) {
                                Some(wallet_balance) => {
                                    *wallet_balance = amount;
                                }
                                None => {
                                    panic!("Wallet type {} does not exist!", wallet_type)
                                }
                            };
                        }
                    }
                }
            }
        }

        db
    }
}

#[cfg(test)]
mod tests {
    use crate::flapjack_stack::flap_sequence_builder::FlapSequenceBuilder;

    #[test]
    fn test_serialization() {
        let log = "
        # the program will register this line a comment
        CREATE \"Checking (Bank)\"
        INCREMENT \"Checking (Bank)\" 50 \"this is a comment for this transactions\"
        ";

        let seq = FlapSequenceBuilder::new(log).build();
        let serialized_first = seq.serialize();

        let seq_rebuilt = FlapSequenceBuilder::new(&serialized_first).build();
        let serialized_again = seq_rebuilt.serialize();

        assert_eq!(serialized_first, serialized_again)
    }

    #[test]
    fn test_db_wallet_creation() {
        let log = "# the program will register this line a comment\nCREATE \"Checking (Bank)\"\nCREATE \"Savings (Bank)\"";
        let seq = FlapSequenceBuilder::new(log).build();

        seq.db
            .wallet_amounts
            .get(&"Checking (Bank)".to_owned())
            .expect("This key does not exist!");
    }

    #[test]
    fn test_db_wallet_increment() {
        let log = "
        CREATE \"Checking (Bank)\"
        CREATE \"Savings (Bank)\"
        INCREMENT \"Checking (Bank)\" 50 \"this is a comment for this transactions\"
        INCREMENT \"Savings (Bank)\" 73
        INCREMENT \"Checking (Bank)\" 25.50 \"this is another comment for the transaction\"
        ";

        let seq = FlapSequenceBuilder::new(log).build();
        match seq.db.wallet_amounts.get("Checking (Bank)") {
            Some(balance) => {
                assert_eq!(*balance, 75.5);
            }
            None => {
                panic!("Wallet does not exist!")
            }
        }

        match seq.db.wallet_amounts.get("Savings (Bank)") {
            Some(balance) => {
                assert_eq!(*balance, 73.0);
            }
            None => {
                panic!("Wallet does not exist!")
            }
        }
    }

    #[test]
    fn test_wallet_set() {
        let log = "
        CREATE \"Checking (Bank)\"
        CREATE \"Savings (Bank)\"
        INCREMENT \"Checking (Bank)\" 50 \"this is a comment for this transactions\"
        INCREMENT \"Savings (Bank)\" 73
        INCREMENT \"Checking (Bank)\" 25.50 \"this is another comment for the transaction\"
        SET \"Savings (Bank)\" 200 \"meow\"
        ";

        let seq = FlapSequenceBuilder::new(log).build();
        match seq.db.wallet_amounts.get("Checking (Bank)") {
            Some(balance) => {
                assert_eq!(*balance, 75.5);
            }
            None => {
                panic!("Wallet does not exist!")
            }
        }

        match seq.db.wallet_amounts.get("Savings (Bank)") {
            Some(balance) => {
                assert_eq!(*balance, 200.0);
            }
            None => {
                panic!("Wallet does not exist!")
            }
        }
    }
}
