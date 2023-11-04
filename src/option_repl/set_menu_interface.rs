use super::{OptionRepl, State};

impl OptionRepl {
    pub(super) fn set_menu_interface(&mut self) {
        let question = "Change amount for which wallet?: ";
        let chosen_wallet_option = self.tell_user_to_pick_wallet(question);

        let chosen_wallet = match chosen_wallet_option {
            Some(x) => x,
            None => {
                self.state = State::Default;
                return;
            }
        };

        println!("Set wallet amount to: ");

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
                    println!("Please enter an integer!");
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
            println!("Is this correct? (Y/n)");
        } else {
            println!("Wallet: {chosen_wallet}");
            println!("Amount: {amount:.2}");
            println!("Comment: (NA)");
            println!("Is this correct? (Y/n)");
        }

        loop {
            let answer = Self::wait_for_input().to_lowercase();
            let trimmed = answer.trim();

            match trimmed {
                "" | "y" => {
                    if !comment.is_empty() {
                        self.stack
                            .set_wallet_amount(&chosen_wallet, amount, Some(&comment));
                    } else {
                        self.stack.set_wallet_amount(&chosen_wallet, amount, None);
                    }

                    println!("Set wallet {}'s amount to {:.2}", chosen_wallet, amount);
                    break;
                }
                "n" => {
                    println!("Did not set wallet amount.");
                    break;
                }
                _ => {
                    println!("Invalid answer! Please answer with 'y' or 'n' or hit enter to accept the default.");
                    continue;
                }
            };
        }

        self.state = State::Default;
    }
}
