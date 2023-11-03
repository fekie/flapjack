use super::{OptionRepl, State};

impl OptionRepl {
    pub(super) fn destroy_menu_interface(&mut self) {
        let question = "Destroy which wallet?: ";
        let chosen_wallet_option = self.tell_user_to_pick_wallet(question);

        let chosen_wallet = match chosen_wallet_option {
            Some(x) => x,
            None => {
                self.state = State::Default;
                return;
            }
        };

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
}
