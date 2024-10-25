use super::Column;
use std::cmp;

impl PartialOrd for Column {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::cmp::Ordering;

    #[test]
    fn partial_cmp() {
        assert_eq!(
            Column::new(5).partial_cmp(&Column::new(3)),
            Some(Ordering::Greater)
        );
        assert_eq!(
            Column::new(5).partial_cmp(&Column::new(5)),
            Some(Ordering::Equal)
        );
        assert_eq!(
            Column::new(5).partial_cmp(&Column::new(100)),
            Some(Ordering::Less)
        );
    }
}
