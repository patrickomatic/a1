use super::Row;
use std::cmp;

impl Ord for Row {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.y.cmp(&other.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmp() {
        assert_eq!(Row::new(20).cmp(&Row::new(20)), cmp::Ordering::Equal);
        assert_eq!(Row::new(20).cmp(&Row::new(19)), cmp::Ordering::Greater);
        assert_eq!(Row::new(20).cmp(&Row::new(21)), cmp::Ordering::Less);
    }
}
