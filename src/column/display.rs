use crate::{Column, ALPHA};
use std::fmt;

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Convert to the "A" part - 0 == 'A', 1 == 'B', etc.  we'll append to a string because
        // if it's larger than 26, we'll have additional characters like AA1
        let mut row_part = String::from("");
        let mut c = self.x;

        loop {
            row_part = format!("{}{}", ALPHA[c % 26], row_part);

            let next_c = ((c as f64 / 26.0).floor() as isize) - 1;
            if next_c < 0 {
                break;
            }

            c = next_c as usize;
        }

        let abs_char = if self.absolute { "$" } else { "" };

        write!(f, "{abs_char}{row_part}")
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn display_relative() {
        assert_eq!(Column::new(0).to_string(), "A");
        assert_eq!(Column::new(1).to_string(), "B");
        assert_eq!(Column::new(2).to_string(), "C");
        assert_eq!(Column::new(25).to_string(), "Z");
        assert_eq!(Column::new(26).to_string(), "AA");
    }

    #[test]
    fn display_absolute() {
        assert_eq!(
            Column {
                absolute: true,
                x: 0
            }
            .to_string(),
            "$A"
        );
        assert_eq!(
            Column {
                absolute: true,
                x: 25
            }
            .to_string(),
            "$Z"
        );
    }
}
