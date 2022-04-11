use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

use self::flapjack::{Command, Directive, FlapJack};

pub mod flapjack;
pub mod flapjack_stack_builder;

/// A sequence of `Flap`s that each contain either a `Directive` or a `Comment`.
/// Each flap in the sequence retains its order.
#[derive(Debug)]
pub struct FlapJackStack {
    pub flapjacks: Vec<FlapJack>,
    pub db: FlapJackDb,
    pub log_path: Option<String>,
}

impl FlapJackStack {
    pub fn new(flapjacks: Vec<FlapJack>, log_path: Option<String>) -> Self {
        let db = FlapJackDb::from_flaps(&flapjacks);
        Self {
            flapjacks,
            db,
            log_path,
        }
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
        for (i, flapjack) in self.flapjacks.iter().enumerate() {
            serialized.push_str(&flapjack.serialize());

            // add a new line if it is not the last line
            if (i + 1) != self.flapjacks.len() {
                serialized.push_str("\n");
            }
        }
        serialized
    }

    pub fn return_wallet_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        for (wallet_name, _amount) in &self.db.wallet_amounts {
            names.push(wallet_name.to_owned());
        }
        names
    }

    pub fn amount(&self, wallet_name: &str) -> f64 {
        let amount = self
            .db
            .wallet_amounts
            .get(wallet_name)
            .expect("This shouldn't be here");

        *amount
    }

    // updates the flap to the db and writes to file
    pub fn push_flap(&mut self, flapjack: FlapJack) {
        self.db.update(&flapjack);
        self.flapjacks.push(flapjack);
        self.serialize_to_file(&self.log_path.clone().expect("This should not happen!"));
    }

    pub fn set_wallet_amount(&mut self, wallet_name: &str, amount: i64, comment: Option<&str>) {
        let flapjack = match comment {
            Some(x) => FlapJack::Directive(Directive {
                command: Command::SET,
                params: vec![wallet_name.to_owned(), amount.to_string(), x.to_owned()],
            }),
            None => FlapJack::Directive(Directive {
                command: Command::SET,
                params: vec![wallet_name.to_owned(), amount.to_string()],
            }),
        };

        self.push_flap(flapjack);
    }

    pub fn decrement_wallet_amount(
        &mut self,
        wallet_name: &str,
        amount: i64,
        comment: Option<&str>,
    ) {
        let flapjack = match comment {
            Some(x) => FlapJack::Directive(Directive {
                command: Command::DECREMENT,
                params: vec![wallet_name.to_owned(), amount.to_string(), x.to_owned()],
            }),
            None => FlapJack::Directive(Directive {
                command: Command::DECREMENT,
                params: vec![wallet_name.to_owned(), amount.to_string()],
            }),
        };

        self.push_flap(flapjack);
    }

    pub fn increment_wallet_amount(
        &mut self,
        wallet_name: &str,
        amount: i64,
        comment: Option<&str>,
    ) {
        let flapjack = match comment {
            Some(x) => FlapJack::Directive(Directive {
                command: Command::INCREMENT,
                params: vec![wallet_name.to_owned(), amount.to_string(), x.to_owned()],
            }),
            None => FlapJack::Directive(Directive {
                command: Command::INCREMENT,
                params: vec![wallet_name.to_owned(), amount.to_string()],
            }),
        };

        self.push_flap(flapjack);
    }

    pub fn create_wallet(&mut self, wallet_name: &str) {
        let flapjack = FlapJack::Directive(Directive {
            command: Command::CREATE,
            params: vec![wallet_name.to_owned()],
        });
        self.push_flap(flapjack);
    }

    pub fn destroy_wallet(&mut self, wallet_name: &str) {
        let flapjack = FlapJack::Directive(Directive {
            command: Command::DESTROY,
            params: vec![wallet_name.to_owned()],
        });
        self.push_flap(flapjack);
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
                FlapJack::Comment(_comment) => {}
                FlapJack::Directive(directive) => {
                    let command = &directive.command;
                    let params = directive.params.as_slice();

                    match command {
                        Command::CREATE => db.command_create(params),
                        Command::INCREMENT => db.command_increment(params),
                        Command::SET => db.command_set(params),
                        Command::DESTROY => db.command_destroy(params),
                        Command::DECREMENT => db.command_decrement(params),
                    }
                }
            }
        }

        db
    }

    // takes a flapjack, updates the db
    pub fn update(&mut self, flap: &FlapJack) {
        match flap {
            FlapJack::Comment(_comment) => {}
            FlapJack::Directive(directive) => {
                let command = &directive.command;
                let params = directive.params.as_slice();

                match command {
                    Command::CREATE => self.command_create(params),
                    Command::INCREMENT => self.command_increment(params),
                    Command::SET => self.command_set(params),
                    Command::DESTROY => self.command_destroy(params),
                    Command::DECREMENT => self.command_decrement(params),
                }
            }
        }
    }

    // TODO: make sure wallet doesnt already exist
    pub fn command_create(&mut self, params: &[String]) {
        let wallet_type = params.get(0).expect("Wallet type argument was not found!");

        self.wallet_amounts.insert(wallet_type.to_string(), 0.0);
    }

    pub fn command_increment(&mut self, params: &[String]) {
        let wallet_type = params.get(0).expect("Wallet type argument was not found!");

        let amount = params
            .get(1)
            .expect("Amount was not found")
            .parse::<f64>()
            .expect("Amount could not be parsed to a float.");

        match self.wallet_amounts.get_mut(&wallet_type.to_string()) {
            Some(wallet_balance) => {
                *wallet_balance += amount;
            }
            None => {
                panic!("Wallet type {} does not exist!", wallet_type)
            }
        };
    }

    pub fn command_set(&mut self, params: &[String]) {
        let wallet_type = params.get(0).expect("Wallet type argument was not found!");

        let amount = params
            .get(1)
            .expect("Amount was not found")
            .parse::<f64>()
            .expect("Amount could not be parsed to a float.");

        match self.wallet_amounts.get_mut(&wallet_type.to_string()) {
            Some(wallet_balance) => {
                *wallet_balance = amount;
            }
            None => {
                panic!("Wallet type {} does not exist!", wallet_type)
            }
        };
    }

    pub fn command_destroy(&mut self, params: &[String]) {
        let wallet_type = params.get(0).expect("Wallet type argument was not found!");
        self.wallet_amounts.remove(wallet_type);
    }

    pub fn command_decrement(&mut self, params: &[String]) {
        let wallet_type = params.get(0).expect("Wallet type argument was not found!");

        let amount = params
            .get(1)
            .expect("Amount was not found")
            .parse::<f64>()
            .expect("Amount could not be parsed to a float.");

        match self.wallet_amounts.get_mut(&wallet_type.to_string()) {
            Some(wallet_balance) => {
                *wallet_balance -= amount;
            }
            None => {
                panic!("Wallet type {} does not exist!", wallet_type)
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::flapjack_stack::flapjack_stack_builder::FlapJackStackBuilder;

    #[test]
    fn test_serialization() {
        let log = "
        # the program will register this line a comment
        CREATE \"Checking (Bank)\"
        INCREMENT \"Checking (Bank)\" 50 \"this is a comment for this transactions\"
        ";

        let seq = FlapJackStackBuilder::new(log, None).build();
        let serialized_first = seq.serialize();

        let seq_rebuilt = FlapJackStackBuilder::new(&serialized_first, None).build();
        let serialized_again = seq_rebuilt.serialize();

        assert_eq!(serialized_first, serialized_again)
    }

    #[test]
    fn test_db_wallet_creation() {
        let log = "# the program will register this line a comment\nCREATE \"Checking (Bank)\"\nCREATE \"Savings (Bank)\"";
        let seq = FlapJackStackBuilder::new(log, None).build();

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

        let seq = FlapJackStackBuilder::new(log, None).build();
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

        let seq = FlapJackStackBuilder::new(log, None).build();
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

    #[test]
    fn test_wallet_destroy() {
        let log = "
        CREATE \"Checking (Bank)\"
        CREATE \"Savings (Bank)\"
        INCREMENT \"Checking (Bank)\" 50 \"this is a comment for this transactions\"
        INCREMENT \"Savings (Bank)\" 73
        INCREMENT \"Checking (Bank)\" 25.50 \"this is another comment for the transaction\"
        SET \"Savings (Bank)\" 200 \"meow\"
        DESTROY \"Savings (Bank)\"
        ";

        let seq = FlapJackStackBuilder::new(log, None).build();

        if let Some(_) = seq.db.wallet_amounts.get("Savings (Bank)") {
            panic!("Wallet was not destroyed!")
        }
    }

    #[test]
    fn test_wallet_decrement() {
        let log = "
        CREATE \"Checking (Bank)\"
        CREATE \"Savings (Bank)\"
        INCREMENT \"Checking (Bank)\" 50 \"this is a comment for this transactions\"
        INCREMENT \"Savings (Bank)\" 73
        INCREMENT \"Checking (Bank)\" 25.50 \"this is another comment for the transaction\"
        SET \"Savings (Bank)\" 200 \"meow\"
        DECREMENT \"Checking (Bank)\" 10.5 \"bought something\"
        ";

        let seq = FlapJackStackBuilder::new(log, None).build();
        match seq.db.wallet_amounts.get("Checking (Bank)") {
            Some(balance) => {
                assert_eq!(*balance, 65.0);
            }
            None => {
                panic!("Wallet does not exist!")
            }
        }
    }
}
