use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    CREATE,
    INCREMENT,
    SET,
    DESTROY,
    DECREMENT,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CREATE => {
                write!(f, "CREATE")
            }
            Self::INCREMENT => {
                write!(f, "INCREMENT")
            }
            Self::SET => {
                write!(f, "SET")
            }
            Self::DESTROY => {
                write!(f, "DESTROY")
            }
            Self::DECREMENT => {
                write!(f, "DECREMENT")
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
/// Contains either a Directive or a Comment
pub enum FlapJack {
    Directive(Directive),
    Comment(Comment),
}

impl FlapJack {
    pub fn serialize(&self) -> String {
        match self {
            Self::Directive(directive) => directive.serialize(),
            Self::Comment(comment) => comment.serialize(),
        }
    }
}

/// A command and params for use in `FlapSequence`s.
/// Follows the pattern Directive { command: "CREATE", params: ["account", "Checking-Bank"] }.
/// Directive structure in the log will look like:
/// INCREMENT checking-bank 46.70 "got paid"
#[derive(Debug, PartialEq, Clone)]
pub struct Directive {
    pub command: Command,
    pub params: Vec<String>,
}

impl Directive {
    pub fn serialize(&self) -> String {
        let mut combined = String::new();
        combined.push_str(&self.command.to_string());
        for param in &self.params {
            combined.push(' ');

            // if the parameter has whitespace, it needs to be surrounded by quotes
            if param.contains(char::is_whitespace) {
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
/// A comment for use in `FlapJackStack`s.
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
