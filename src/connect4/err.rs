use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum C4eParseError {
    WrongNumberOfColumns,
    OverfilledColumn,
    IllegalCharacter,
    InvalidBoard,
}
impl Error for C4eParseError {}
impl Display for C4eParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use C4eParseError::*;
        match self {
            WrongNumberOfColumns => "wrong number of columns".fmt(f),
            OverfilledColumn => "overfilled column".fmt(f),
            IllegalCharacter => "illegal character".fmt(f),
            InvalidBoard => "invalid board".fmt(f),
        }
    }
}
