use crate::{Error, RangeOrCell, Result, A1};
use std::str;

fn parse_quoted_sheet_name(a1: &str) -> Result<(Option<String>, &str)> {
    let mut unquoted = String::new();
    let mut saw_start_quote = false;
    let mut last_was_quote = false;
    let mut consumed = 0;

    for (i, c) in a1.chars().enumerate() {
        if c == '\'' {
            // two quotes in a row means it's a quoted quote
            if last_was_quote {
                unquoted.push(c);
                last_was_quote = false;
            } else if !saw_start_quote {
                saw_start_quote = true;
            } else {
                last_was_quote = true;
            }
        } else {
            if last_was_quote {
                consumed = i;
                break;
            }
            unquoted.push(c);
        }
    }

    if consumed == 0 {
        return Err(Error::parse_error(a1, "Expected a single-quoted string"));
    } else if !a1[consumed..].starts_with('!') {
        return Err(Error::parse_error(
            a1,
            "Expected a `!` after the single quoted name",
        ));
    }

    // consumed + 1 so we skip the `!` char
    Ok((Some(unquoted), &a1[(consumed + 1)..]))
}

fn parse_sheet_name(a1: &str) -> Result<(Option<String>, &str)> {
    let trimmed_a1 = a1.trim_start();
    if trimmed_a1.starts_with('\'') {
        parse_quoted_sheet_name(trimmed_a1)
    } else if let Some((sheet_name, rest)) = a1.split_once('!') {
        Ok((Some(sheet_name.to_string()), rest))
    } else {
        Ok((None, a1))
    }
}

impl str::FromStr for A1 {
    type Err = Error;

    fn from_str(a1: &str) -> Result<Self> {
        let (sheet_name, rest) = parse_sheet_name(a1)?;
        let reference = RangeOrCell::from_str(rest)?;

        Ok(A1 {
            sheet_name,
            reference,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn from_str() {
        assert_eq!(
            A1 {
                sheet_name: None,
                reference: RangeOrCell::Cell((0, 0).into()),
            },
            A1::from_str("A1").unwrap()
        );
    }

    #[test]
    fn from_str_sheet_name() {
        assert_eq!(
            A1 {
                sheet_name: Some("Foo".to_string()),
                reference: RangeOrCell::Cell((0, 0).into()),
            },
            A1::from_str("Foo!A1").unwrap()
        );
    }

    #[test]
    fn from_str_sheet_name_quotes() {
        assert_eq!(
            A1 {
                sheet_name: Some("Foo Bar".to_string()),
                reference: RangeOrCell::Cell((0, 0).into()),
            },
            A1::from_str("'Foo Bar'!A1").unwrap()
        );
    }

    #[test]
    fn from_str_sheet_name_quotes_escape() {
        assert_eq!(
            A1 {
                sheet_name: Some("Foo\'s Bar".to_string()),
                reference: RangeOrCell::Cell((0, 0).into()),
            },
            A1::from_str("'Foo''s Bar'!A1").unwrap()
        );
    }

    #[test]
    fn from_str_sheet_name_invalid() {
        // no closing quote
        assert!(A1::from_str("'Foo''s Bar!A1").is_err());

        // no ! after the sheet name
        assert!(A1::from_str("'Foo Bar'").is_err());
    }
}
