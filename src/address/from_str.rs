use crate::{Address, Column, Error, Result, Row};
use std::str;

impl str::FromStr for Address {
    type Err = Error;

    fn from_str(a1: &str) -> Result<Self> {
        let mut split_at = 0;

        for (i, c) in a1.chars().enumerate() {
            if (c == '$' && i > 0) || c.is_ascii_digit() {
                split_at = i;
                break;
            }
        }

        if split_at == 0 {
            return Err(Error::parse_error(
                a1,
                "You must supply a valid A1 reference with at least one letter followed by a number.",
            ));
        }

        Ok(Self {
            column: Column::from_str(&a1[..split_at])?,
            row: Row::from_str(&a1[split_at..])?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn from_str_ok() {
        assert_eq!(Address::new(0, 0), Address::from_str("A1").unwrap());
        assert_eq!(Address::new(50, 50), Address::from_str("AY51").unwrap());
    }

    #[test]
    fn from_str_absolute() {
        let a1 = Address::from_str("$A$1").unwrap();
        assert!(a1.column.absolute);
        assert!(a1.row.absolute);

        let ay51 = Address::from_str("AY$51").unwrap();
        assert!(!ay51.column.absolute);
        assert!(ay51.row.absolute);
    }

    #[test]
    fn from_str_err() {
        assert!(Address::from_str("").is_err());
        assert!(Address::from_str("/foo").is_err());
    }
}
