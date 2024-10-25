//! # Error
use std::fmt;

#[derive(Clone, Debug)]
pub enum Error {
    /// # A1ParseError
    ///
    /// An error encountered by parsing a String into A1 format.
    ///
    /// * `bad_input` - The offending input that could not be parsed.
    /// * `message` - A relevant error message.
    A1ParseError { bad_input: String, message: String },
}

impl Error {
    pub(crate) fn parse_error<A: Into<String>, B: Into<String>>(bad_input: A, message: B) -> Self {
        Self::A1ParseError {
            bad_input: bad_input.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::A1ParseError { message, bad_input } => {
                write!(f, "{message} (input: `{bad_input}`)")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_a1_parse_error() {
        assert_eq!(
            Error::A1ParseError {
                message: "Foo was a bar".to_string(),
                bad_input: "bar".to_string(),
            }
            .to_string(),
            "Foo was a bar (input: `bar`)"
        );
    }
}
