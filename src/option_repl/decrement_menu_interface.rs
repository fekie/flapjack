use super::{OptionRepl, State};

impl OptionRepl {
    pub(super) fn decrement_menu_interface(&mut self) {
        let question = "Decrement amount for which wallet?: ";
        let chosen_wallet_option = self.tell_user_to_pick_wallet(question);

        let chosen_wallet = match chosen_wallet_option {
            Some(x) => x,
            None => {
                self.state = State::Default;
                return;
            }
        };

        println!("Decrement wallet amount by: ");

        let amount: f64;
        loop {
            let input = Self::wait_for_input();
            match input.parse::<f64>() {
                Ok(x) => {
                    amount = x;
                    break;
                }
                Err(_) => {
                    Self::print_divider();
                    println!("Please enter a number!");
                    continue;
                }
            };
        }

        println!("Enter comment: ");
        let comment = Self::wait_for_input();

        if !comment.is_empty() {
            println!("Wallet: {chosen_wallet}");
            println!("Amount: {amount:.2}");
            println!("Comment: \"{comment}\"");
            println!("Is this correct? (y/n)");
        } else {
            println!("Wallet: {chosen_wallet}");
            println!("Amount: {amount:.2}");
            println!("Comment: (NA)");
            println!("Is this correct? (y/n)");
        }

        loop {
            let answer: &str = &Self::wait_for_input();
            match answer {
                "y" => {
                    if !comment.is_empty() {
                        self.stack
                            .decrement_wallet_amount(&chosen_wallet, amount, Some(&comment));
                    } else {
                        self.stack
                            .decrement_wallet_amount(&chosen_wallet, amount, None);
                    }

                    println!(
                        "Decremented wallet {}'s amount by {:.2}.",
                        chosen_wallet, amount
                    );
                    break;
                }
                "n" => {
                    println!("Did not decrement wallet amount.");
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
}
