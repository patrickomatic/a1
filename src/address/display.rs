use super::Address;
use std::fmt;

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.column, self.row)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!("A1", Address::new(0, 0).to_string());
        assert_eq!("C5", Address::new(2, 4).to_string());
        assert_eq!("AY51", Address::new(50, 50).to_string());
    }
}
