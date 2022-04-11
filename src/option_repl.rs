use crate::flapjack_stack::flapjack::{Command, Comment, Directive, FlapJack};
use crate::flapjack_stack::FlapJackStack;
use prettytable::{Cell, Row, Table};
use std::io::stdin;
use std::io::{self, Write};
use std::process::exit;

pub enum State {
    Default,
    View,
    Invalid,
    Exit,
    CreateMenu,
    IncrementMenu,
    SetMenu,
    DestroyMenu,
    DecrementMenu,
}

pub struct OptionRepl {
    stack: FlapJackStack,
    state: State,
}

impl OptionRepl {
    pub fn new(stack: FlapJackStack) -> Self {
        Self {
            stack,
            state: State::Default,
        }
    }

    pub fn start(mut self) {
        loop {
            self.next()
        }
    }

    fn next(&mut self) {
        Self::print_divider();
        match &self.state {
            State::Default => self.handle_default(),
            State::View => self.view(),
            State::Invalid => self.invalid(),
            State::Exit => self.exit(),
            State::CreateMenu => self.create_menu_interface(),
            State::DestroyMenu => self.destroy_menu_interface(),
            _ => unimplemented!(),
        }
    }

    fn handle_default(&mut self) {
        println!("Options: Set[0] Increment[1] Decrement[2] Create[3] Destroy[4] View[5] Exit[6]");
        let input = Self::wait_for_input();

        if let Err(_) = input.parse::<u64>() {
            Self::print_divider();
            println!("Please enter an integer!");
            return;
        }

        let choice_num = input.parse::<u64>().unwrap();

        match choice_num {
            3 => self.state = State::CreateMenu,
            4 => self.state = State::DestroyMenu,
            5 => self.state = State::View,
            6 => self.state = State::Exit,
            _ => self.state = State::Invalid,
        };
    }

    fn destroy_menu_interface(&mut self) {
        let mut print_str = "Destroy which wallet?: ".to_owned();
        let wallet_names = self.stack.return_wallet_names();
        let wallet_name_count = wallet_names.len() as i64;
        for (i, wallet_name) in wallet_names.iter().enumerate() {
            print_str.push_str(wallet_name);
            print_str.push('[');
            print_str.push_str(&i.to_string());
            print_str.push(']');
            if (i + 1) as i64 != wallet_name_count {
                print_str.push(' ')
            } else {
                print_str.push_str(" BACK[");
                print_str.push_str(&(i + 1).to_string());
                print_str.push(']');
            }
        }

        let minimum = 0 as i64;
        let maximum = wallet_name_count;

        // wait until a valid option is chosen
        let choice_num: i64;
        loop {
            println!("{}", print_str);
            let input = Self::wait_for_input();
            if let Err(_) = input.parse::<u64>() {
                Self::print_divider();
                println!("Please enter an integer!");
                continue;
            }

            let num = input.parse::<i64>().unwrap();
            if (num < minimum) || (num > maximum) {
                Self::print_divider();
                println!("Invalid option!");
                continue;
            }

            choice_num = num;
            break;
        }

        // check to see if they wanted to go back
        if choice_num == wallet_name_count {
            self.state = State::Default;
            return;
        }

        let chosen_wallet = &wallet_names[choice_num as usize];
        println!(
            "The wallet {} will be destroyed. Confirm? (y/n)",
            chosen_wallet
        );

        loop {
            let answer: &str = &Self::wait_for_input();
            match answer {
                "y" => {
                    self.stack.destroy_wallet(&chosen_wallet);
                    println!("Destroyed wallet: {}", chosen_wallet);
                    break;
                }
                "n" => {
                    println!("Did not destroy wallet.");
                    break;
                }
                _ => {
                    println!("Invalid answer! Please answer with 'y' or 'n'.");
                    continue;
                }
            };
        }

        self.state = State::Default;
    }

    fn create_menu_interface(&mut self) {
        println!("Wallet Name: ");
        let name = Self::wait_for_input();

        println!("The wallet will be named {}. Confirm? (y/n)", name);

        loop {
            let answer: &str = &Self::wait_for_input();
            match answer {
                "y" => {
                    self.stack.create_wallet(&name);
                    println!("Created wallet: {}", name);
                    break;
                }
                "n" => {
                    println!("Did not create wallet.");
                    break;
                }
                _ => {
                    println!("Invalid answer! Please answer with 'y' or 'n'.");
                    continue;
                }
            };
        }

        self.state = State::Default;
    }

    fn exit(&self) {
        exit(0)
    }

    fn invalid(&mut self) {
        println!("Invalid option!");
        self.state = State::Default
    }

    fn view(&mut self) {
        // Create the table
        let mut table = Table::new();

        // TODO: add time later
        table.add_row(row!["Wallet", "Amount"]);
        let wallet_names = self.stack.return_wallet_names();
        for name in wallet_names.iter() {
            let amount = self.stack.amount(&name);
            table.add_row(row![name, amount]);
        }

        // Print the table to stdout
        table.printstd();
        self.state = State::Default;
    }

    fn wait_for_input() -> String {
        print!(">>> ");
        io::stdout().flush().unwrap();
        let mut input_string = String::new();
        stdin()
            .read_line(&mut input_string)
            .ok()
            .expect("Failed to read line");

        input_string.trim().to_owned()
    }

    pub fn print_divider() {
        println!("------------------------------------")
    }
}
