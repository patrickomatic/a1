use crate::{Column, Error, ALPHA};
use std::str::FromStr;

impl FromStr for Column {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut absolute = false;
        let ys = if let Some(without_abs) = s.strip_prefix('$') {
            absolute = true;
            without_abs
        } else {
            s
        };

        let mut x = 0;
        for ch in ys.chars() {
            let uch = ch.to_ascii_uppercase();
            if let Some(ch_index) = ALPHA.iter().position(|&c| c == uch) {
                x = x * 26 + ch_index + 1;
            } else {
                return Err(Error::parse_error(
                    ch,
                    format!("Invalid character in A1 notation: {s}"),
                ));
            }
        }

        Ok(Self { absolute, x: x - 1 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_ok() {
        let a = Column::from_str("A").unwrap();
        assert_eq!(a.x, 0);
        assert!(!a.absolute);

        let z = Column::from_str("Z").unwrap();
        assert_eq!(z.x, 25);
        assert!(!z.absolute);
    }

    #[test]
    fn from_str_ok_absolute() {
        let a = Column::from_str("$A").unwrap();
        assert_eq!(a.x, 0);
        assert!(a.absolute);

        let z = Column::from_str("$Z").unwrap();
        assert_eq!(z.x, 25);
        assert!(z.absolute);
    }

    #[test]
    fn from_str_err() {
        assert!(Column::from_str("123").is_err());
        assert!(Column::from_str("<foo>").is_err());
    }
}
