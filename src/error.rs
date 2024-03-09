use std::fmt;

pub enum Error<'a> {
    UnexpectedCharacter(char, usize),
    InvalidTag(&'a str),
    ExtraTags(Vec<&'a str>),
}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedCharacter(c, i) => write!(f, "Unexpected '{c}' at {i}"),
            Self::InvalidTag(tag) => write!(f, "Invalid tag <{tag}>"),
            Self::ExtraTags(tags) => write!(f, "Extra tags {:?} not closed", tags),
        }
    }
}
