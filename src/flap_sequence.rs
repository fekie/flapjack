use regex::Regex;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Clone)]
/// A command and params for use in `FlapSequence`s.
/// Follows the pattern Directive { command: "CREATE", params: ["account", "Checking-Bank"] }.
/// Directive structure in the log will look like:
/// INCREMENT checking-bank 46.70 "got paid"
pub struct Directive {
    pub command: String,
    pub params: Vec<String>,
}

impl Directive {
    pub fn serialize(&self) -> String {
        let mut combined = String::new();
        combined.push_str(&self.command.clone());
        for param in &self.params {
            combined.push(' ');

            // if the parameter has whitespace, it needs to be surrounded by quotes
            if param.contains(char::is_whitespace) {
                println!("{}", param);
                let mut param_with_quotes = String::from('\"');
                param_with_quotes.push_str(&param);
                param_with_quotes.push('\"');
                combined.push_str(&param_with_quotes);
            } else {
                combined.push_str(&param.clone());
            }
        }

        combined
    }
}

#[derive(Debug, PartialEq, Clone)]
/// A comment for use in `FlapSequence`s.
/// Follows the pattern Comment("# this is a comment");
pub struct Comment {
    pub string: String,
}

impl Comment {
    pub fn new(s: String) -> Comment {
        Self { string: s }
    }

    pub fn serialize(&self) -> String {
        self.string.clone()
    }
}

#[derive(Debug, PartialEq, Clone)]
/// Contains either a Directive or a Comment
pub enum Flap {
    Directive(Directive),
    Comment(Comment),
}

impl Flap {
    pub fn serialize(&self) -> String {
        match self {
            Self::Directive(directive) => directive.serialize(),
            Self::Comment(comment) => comment.serialize(),
        }
    }
}

/// A sequence of `Flap`s that each contain either a `Directive` or a `Comment`.
/// Each flap in the sequence retains its order.
#[warn(missing_docs)]
#[derive(Debug)]
pub struct FlapSequence {
    /// A `Vector` of `Flap`s
    pub flaps: Vec<Flap>,
}

impl FlapSequence {
    pub fn serialize_to_file(&self, path: &str) {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(path)
            .unwrap();

        let serialized = self.serialize();
        file.write(serialized.as_bytes()).unwrap();
    }

    fn serialize(&self) -> String {
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

#[warn(missing_docs)]
#[derive(Debug)]
/// A builder to help create a `FlapSequence`.
/// This builder takes a raw string, removes carriage returns, splits it by lines,
/// parses the lines into `Comment`s and `Directive`s, and creates a `FlapSequence`.
pub struct FlapSequenceBuilder {
    lines: Vec<String>,
}

impl FlapSequenceBuilder {
    pub fn new(raw_log: String) -> Self {
        // go through a parsing process
        let lines = Self::split_and_clean_raw_log(raw_log);
        Self { lines }
    }

    pub fn from_file(path: &str) -> Self {
        let file = fs::read_to_string(path);
        let content = file.expect(&format!("Can't find file {}", path));
        Self::new(content)
    }

    pub fn build(&mut self) -> FlapSequence {
        let mut flaps: Vec<Flap> = Vec::new();

        for mut line in self.lines.drain(..) {
            // this is the regex for splitting on whitespace, unless something is in quotations
            let mut split = Self::split_and_clean_line(&mut line);

            let flap = match line.chars().nth(0).unwrap() {
                // line is a comment
                '#' => {
                    let comment = Comment::new(line.to_string());
                    Flap::Comment(comment)
                }
                // line is a directive
                _ => {
                    let command = split.remove(0);
                    let directive = Directive {
                        command: command,
                        params: split,
                    };

                    Flap::Directive(directive)
                }
            };

            flaps.push(flap)
        }

        FlapSequence { flaps }
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

    fn split_and_clean_raw_log(raw_log: String) -> Vec<String> {
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
mod tests {
    use crate::flap_sequence::{Comment, Directive, Flap, FlapSequenceBuilder};

    #[test]
    fn test_for_carriage_return_discrimination() {
        let log_with_carriage_returns = "# the program will register this line a comment\r
        CREATE account \"Checking (Bank)\"\r
        CREATE account \"Savings (Bank)\"\r";

        let log_without_carriage_returns = "# the program will register this line a comment
        CREATE account \"Checking (Bank)\"
        CREATE account \"Savings (Bank)\"";

        let parsed_carriage =
            FlapSequenceBuilder::split_and_clean_raw_log(log_with_carriage_returns.to_owned());

        let parsed_no_carriage =
            FlapSequenceBuilder::split_and_clean_raw_log(log_without_carriage_returns.to_owned());

        assert_eq!(parsed_carriage, parsed_no_carriage);
    }

    #[test]
    fn test_builder() {
        let log = "# the program will register this line a comment
            CREATE account \"Checking (Bank)\"
            CREATE account \"Savings (Bank)\"";

        let seq = FlapSequenceBuilder::new(log.to_string()).build();

        assert_eq!(
            seq.flaps[0],
            Flap::Comment(Comment::new(
                "# the program will register this line a comment".to_owned()
            ))
        );

        assert_eq!(
            seq.flaps[1],
            Flap::Directive(Directive {
                command: "CREATE".to_owned(),
                params: vec!["account".to_owned(), "Checking (Bank)".to_owned()]
            })
        );

        assert_eq!(
            seq.flaps[2],
            Flap::Directive(Directive {
                command: "CREATE".to_owned(),
                params: vec!["account".to_owned(), "Savings (Bank)".to_owned()]
            })
        );
    }

    #[test]
    fn test_serialization() {
        let log = "# the program will register this line a comment\nCREATE account \"Checking (Bank)\"\nCREATE account \"Savings (Bank)\"";

        let seq = FlapSequenceBuilder::new(log.to_string()).build();
        let serialized = seq.serialize();

        assert_eq!(serialized, log);

        let seq_rebuilt = FlapSequenceBuilder::new(serialized.clone()).build();
        let serialized_again = seq_rebuilt.serialize();

        assert_eq!(serialized, serialized_again)
    }
}
