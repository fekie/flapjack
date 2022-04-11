use crate::flapjack_stack::flapjack::{Command, Comment, Directive, FlapJack};
use crate::flapjack_stack::FlapJackStack;
use std::io::{self, Write};

use std::io::stdin;
use std::process::exit;

// the u8 is the "stage" of the menu
pub enum State {
    Default,
    CreateMenu(u8),
    IncrementMenu(u8),
    SetMenu(u8),
    DestroyMenu(u8),
    DecrementMenu(u8),
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
            _ => unimplemented!(),
        }
    }

    fn handle_default(&mut self) {
        println!("{}", Self::create_handle_default_string());
        let input = Self::wait_for_input();

        if let Err(_) = input.parse::<u64>() {
            self.state = State::Default;
            println!("Please enter an integer!");
            return;
        }

        let choice_num = input.parse::<u64>().unwrap();

        // make the logic happen here
        let next_state = match choice_num {
            0 => exit(0),
            _ => {
                println!("Invalid option!");
                State::Default
            }
        };

        self.state = next_state
    }

    fn create_handle_default_string() -> String {
        let foo = "Options: Exit[0]";
        foo.to_owned()
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
