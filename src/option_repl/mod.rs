use crate::flapjack_stack::FlapJackStack;
use prettytable::Table;
use std::io::stdin;
use std::io::{self, Write};
use std::process::exit;

mod create_menu_interface;
mod decrement_menu_interface;
mod destroy_menu_interface;
mod increment_menu_interface;
mod set_menu_interface;

const VALID_STATES: [State; 7] = [
    State::SetMenu,
    State::IncrementMenu,
    State::DecrementMenu,
    State::CreateMenu,
    State::DestroyMenu,
    State::View,
    State::Exit,
];

#[derive(Clone, Copy)]
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
        Self::print_divider();
        println!(
            "Loaded data from {}",
            self.stack
                .log_path
                .clone()
                .expect("There needs to be a log somewhere!")
        );

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
            State::SetMenu => self.set_menu_interface(),
            State::IncrementMenu => self.increment_menu_interface(),
            State::DecrementMenu => self.decrement_menu_interface(),
        }
    }

    fn handle_default(&mut self) {
        println!("Options: Set[0] Increment[1] Decrement[2] Create[3] Destroy[4] View[5] Exit[6]");

        let input = Self::wait_for_input();

        let choice_num = match input.parse::<u64>() {
            Ok(x) => x,
            Err(_) => {
                Self::print_divider();
                println!("Please enter an integer!");
                return;
            }
        };

        // match the choice based on the number chosen
        // this is the same as a match, just smaller
        self.state = *VALID_STATES
            .get(choice_num as usize)
            .unwrap_or(&State::Invalid);
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

        let mut total = 0.0;
        table.add_row(row!["Wallet", "Amount"]);
        let wallet_names = self.stack.return_wallet_names();
        for name in wallet_names.iter() {
            let amount = self.stack.amount(name);
            total += amount;
            // We make sure that the amount only has 2 decimal places
            table.add_row(row![name, format!("{amount:.2}")]);
        }

        // We make sure that the total only has 2 decimal places
        table.add_row(row!["Total", format!("{total:.2}")]);

        // DO NOT USE table.printstd() IT DOES NOT WORK RIGHT ON WINDOWS
        let str = table.to_string();
        print!("{str}");
        io::stdout().flush().unwrap();
        self.state = State::Default;
    }

    fn wait_for_input() -> String {
        print!(">>> ");
        io::stdout().flush().unwrap();
        let mut input_string = String::new();
        stdin()
            .read_line(&mut input_string)
            .expect("Failed to read line");

        input_string.trim().to_owned()
    }

    fn print_divider() {
        println!("------------------------------------")
    }

    // return a Some<wallet_name> if a wallet was chosen, None if they chose back
    fn tell_user_to_pick_wallet(&self, question: &str) -> Option<String> {
        let mut print_str = question.to_owned();
        let wallet_names = self.stack.return_wallet_names();
        let wallet_name_count = wallet_names.len() as i64;

        for (i, wallet_name) in wallet_names.iter().enumerate() {
            print_str.push_str(wallet_name);
            print_str.push('[');
            print_str.push_str(&i.to_string());
            print_str.push(']');
            if (i + 1) as i64 != wallet_name_count {
                print_str.push(' ')
            }
        }

        print_str.push_str(" BACK[");
        print_str.push_str(&(wallet_name_count).to_string());
        print_str.push(']');

        let minimum = 0;
        let maximum = wallet_name_count;

        // wait until a valid option is chosen
        let choice_num: i64;
        loop {
            println!("{}", print_str);
            let input = Self::wait_for_input();
            if input.parse::<f64>().is_err() {
                Self::print_divider();
                println!("Please enter a number!");
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
            None
        } else {
            let chosen_wallet = &wallet_names[choice_num as usize];
            Some(chosen_wallet.clone())
        }
    }
}
