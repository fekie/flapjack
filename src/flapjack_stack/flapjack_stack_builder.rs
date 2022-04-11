use regex::Regex;
use std::fs;

use crate::flapjack_stack::flapjack::{Command, Comment, Directive, FlapJack};
use crate::flapjack_stack::FlapJackStack;

/// A builder to help create a `FlapJackStack`.
/// This builder takes a raw string, removes carriage returns, splits it by lines,
/// parses the lines into `Comment`s and `Directive`s, and creates a `FlapJackStack`.
#[derive(Debug)]
pub struct FlapJackStackBuilder {
    lines: Vec<String>,
    log_path: Option<String>,
}

impl FlapJackStackBuilder {
    pub fn new(raw_log: &str, log_path: Option<String>) -> Self {
        // go through a parsing process
        let lines = Self::split_and_clean_raw_log(raw_log);
        Self { lines, log_path }
    }

    pub fn from_file(path: &str) -> Self {
        let file = fs::read_to_string(path);
        let content = file.expect(&format!("Can't find file {}", path));
        Self::new(&content, Some(path.to_owned()))
    }

    pub fn build(&mut self) -> FlapJackStack {
        let mut flapjacks: Vec<FlapJack> = Vec::new();

        for mut line in self.lines.drain(..) {
            // this is the regex for splitting on whitespace, unless something is in quotations
            let mut split = Self::split_and_clean_line(&mut line);

            let flapjack = match split[0].chars().nth(0).unwrap() {
                // line is a comment
                '#' => {
                    let comment = Comment::new(line.to_string());
                    FlapJack::Comment(comment)
                }
                // line is a directive
                _ => {
                    let command_string: &str = &split.remove(0);
                    let command = match command_string {
                        "CREATE" => Command::CREATE,
                        "INCREMENT" => Command::INCREMENT,
                        "SET" => Command::SET,
                        "DESTROY" => Command::DESTROY,
                        "DECREMENT" => Command::DECREMENT,
                        _ => panic!("Command \"{}\" does not exist!", command_string),
                    };
                    let directive = Directive {
                        command: command,
                        params: split,
                    };

                    FlapJack::Directive(directive)
                }
            };

            flapjacks.push(flapjack)
        }

        FlapJackStack::new(flapjacks, self.log_path.clone())
    }

    fn split_and_clean_line(line: &mut String) -> Vec<String> {
        let re = Regex::new(r#"[^\s"']+|"([^"]*)"|'([^']*)'"#).unwrap();
        let mut split = re
            .find_iter(&line)
            .filter_map(|chunk| Some(chunk.as_str().to_owned()))
            .collect::<Vec<String>>();

        // remove any quotes left
        for part in split.iter_mut() {
            let split_on_quotes = part.split("\"");
            *part = split_on_quotes.collect();
        }
        split
    }

    fn split_and_clean_raw_log(raw_log: &str) -> Vec<String> {
        let no_carriage_returns = Self::remove_carriage_returns(&raw_log);
        let split = no_carriage_returns.split("\n").collect::<Vec<&str>>();
        let mut cleaned: Vec<String> = Vec::new();

        for line in split {
            if Self::remove_whitespace(line) == "" {
                continue;
            }
            cleaned.push(line.to_owned());
        }

        cleaned
    }

    fn remove_carriage_returns(s: &str) -> String {
        s.split("\r").collect()
    }

    fn remove_whitespace(s: &str) -> String {
        s.split_whitespace().collect()
    }
}

#[cfg(test)]
mod test {
    use crate::flapjack_stack::flapjack::{Command, Comment, Directive, FlapJack};
    use crate::flapjack_stack::flapjack_stack_builder::FlapJackStackBuilder;

    #[test]
    fn test_for_carriage_return_discrimination() {
        let log_with_carriage_returns = "# the program will register this line a comment\r
        CREATE account \"Checking (Bank)\"\r
        CREATE account \"Savings (Bank)\"\r";

        let log_without_carriage_returns = "# the program will register this line a comment
        CREATE account \"Checking (Bank)\"
        CREATE account \"Savings (Bank)\"";

        let parsed_carriage =
            FlapJackStackBuilder::split_and_clean_raw_log(log_with_carriage_returns);

        let parsed_no_carriage =
            FlapJackStackBuilder::split_and_clean_raw_log(log_without_carriage_returns);

        assert_eq!(parsed_carriage, parsed_no_carriage);
    }

    #[test]
    fn test_builder() {
        let log = "# the program will register this line a comment
            CREATE account \"Checking (Bank)\"
            CREATE account \"Savings (Bank)\"";

        let stack = FlapJackStackBuilder::new(log, None).build();

        assert_eq!(
            stack.flapjacks[0],
            FlapJack::Comment(Comment::new(
                "# the program will register this line a comment".to_owned()
            ))
        );

        assert_eq!(
            stack.flapjacks[1],
            FlapJack::Directive(Directive {
                command: Command::CREATE,
                params: vec!["account".to_owned(), "Checking (Bank)".to_owned()]
            })
        );

        assert_eq!(
            stack.flapjacks[2],
            FlapJack::Directive(Directive {
                command: Command::CREATE,
                params: vec!["account".to_owned(), "Savings (Bank)".to_owned()]
            })
        );
    }
}
