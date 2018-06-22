use std::error::Error;
use std::fmt;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum MalType {
    Nil,
    True,
    False,
    Number(i64),
    Keyword(String),
    String(String),
    Symbol(String),
    List(Vec<MalType>),
    Vector(Vec<MalType>),
    HashMap(BTreeMap<MalType, MalType>),
}

pub type MalResult = Result<MalType, MalError>;

#[derive(Debug, PartialEq)]
pub enum MalError {
    Parse(String),
    BlankLine,
}

impl fmt::Display for MalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MalError::Parse(ref msg) => write!(f, "Parse error: {}", msg),
            MalError::BlankLine => write!(f, "Blank line"),
        }
    }
}

impl Error for MalError {
    fn description(&self) -> &str {
        match *self {
            MalError::Parse(ref msg) => msg,
            MalError::BlankLine => "Blank line",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
