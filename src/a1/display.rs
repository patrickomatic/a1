use crate::A1;
use std::fmt;

fn escape_quotes(sheet_name: &str) -> String {
    sheet_name.replace('\'', "\\'")
}

fn needs_quotes(sheet_name: &str) -> bool {
    for c in sheet_name.chars() {
        if c.is_whitespace() || c == '\'' {
            return true;
        }
    }
    false
}

impl fmt::Display for A1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = &self.reference;
        if let Some(sheet_name) = &self.sheet_name {
            if needs_quotes(sheet_name) {
                write!(f, "'{}'!{r}", escape_quotes(sheet_name))
            } else {
                write!(f, "{sheet_name}!{r}")
            }
        } else {
            write!(f, "{r}")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn display() {
        let a1 = A1 {
            sheet_name: Some("Test1".to_string()),
            reference: RangeOrCell::Cell((1, 1).into()),
        };

        assert_eq!("Test1!B2", a1.to_string());
    }

    #[test]
    fn display_quoted_sheet_name() {
        let a1 = A1 {
            sheet_name: Some("Foo Bar".to_string()),
            reference: RangeOrCell::Cell((1, 1).into()),
        };

        assert_eq!("'Foo Bar'!B2", a1.to_string());
    }

    #[test]
    fn display_without_sheet_name() {
        let a1 = A1 {
            sheet_name: None,
            reference: RangeOrCell::Cell((0, 0).into()),
        };

        assert_eq!("A1", a1.to_string());
    }

    #[test]
    fn display_range() {
        let a1 = A1 {
            sheet_name: None,
            reference: RangeOrCell::ColumnRange {
                from: 1.into(),
                to: 5.into(),
            },
        };

        assert_eq!("B:F", a1.to_string());
    }
}
