use std::str::FromStr;

use crate::parse;

#[derive(Debug, PartialEq)]
pub enum Command {
    Get(String),
    Set(String, String),
    Del(String),
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s)
    }
}
