use super::Row;
use std::fmt;

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let row_num = self.y + 1;
        let abs_char = if self.absolute { "$" } else { "" };

        write!(f, "{abs_char}{row_num}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_relative() {
        assert_eq!(Row::new(0).to_string(), "1");
        assert_eq!(Row::new(15).to_string(), "16");
        assert_eq!(Row::new(42).to_string(), "43");
        assert_eq!(Row::new(100).to_string(), "101");
    }

    #[test]
    fn display_absolute() {
        assert_eq!(
            Row {
                y: 0,
                absolute: true
            }
            .to_string(),
            "$1"
        );
        assert_eq!(
            Row {
                y: 10,
                absolute: true
            }
            .to_string(),
            "$11"
        );
    }
}
