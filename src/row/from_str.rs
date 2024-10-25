use crate::{Error, Index, Row};
use std::str::FromStr;

/// Parses *just* the "1" part of an "A1" reference.  Which would be a number, possibly prefixed
/// with `$`.  Any other input that is supplied will throw an error.
impl FromStr for Row {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut absolute = false;

        let ys = if let Some(without_abs) = s.strip_prefix('$') {
            absolute = true;
            without_abs
        } else {
            s
        };

        let y = ys.parse::<Index>().map_err(|e| {
            Error::parse_error(
                s,
                format!("Error parsing number part of A1 reference: {e:?}"),
            )
        })?;

        if y < 1 {
            return Err(Error::parse_error(
                y.to_string(),
                "A1 reference must be greater than 0",
            ));
        }

        Ok(Self { absolute, y: y - 1 })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn from_str_ok() {
        assert_eq!(Row::from_str("1").unwrap(), Row::new(0));
        assert_eq!(Row::from_str("124").unwrap(), Row::new(123));
    }

    #[test]
    fn from_str_ok_absolute() {
        assert_eq!(
            Row::from_str("$1").unwrap(),
            Row {
                y: 0,
                absolute: true
            }
        );
        assert_eq!(
            Row::from_str("$124").unwrap(),
            Row {
                y: 123,
                absolute: true
            }
        );
    }

    #[test]
    fn from_str_error() {
        assert!(Row::from_str("ABC").is_err());
        assert!(Row::from_str(" ! < ").is_err());
    }
}
