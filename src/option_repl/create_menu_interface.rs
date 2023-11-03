use super::{OptionRepl, State};

impl OptionRepl {
    pub(super) fn create_menu_interface(&mut self) {
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
}
