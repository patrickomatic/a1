use super::Column;
use std::cmp;

impl Ord for Column {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.x.cmp(&other.x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmp() {
        assert_eq!(Column::new(20).cmp(&Column::new(20)), cmp::Ordering::Equal);
        assert_eq!(
            Column::new(20).cmp(&Column::new(19)),
            cmp::Ordering::Greater
        );
        assert_eq!(Column::new(20).cmp(&Column::new(21)), cmp::Ordering::Less);
    }
}
