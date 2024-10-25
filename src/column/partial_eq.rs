use super::Column;

impl PartialEq for Column {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq() {
        assert_eq!(Column::new(0), Column::new(0));
        assert_eq!(Column::new(100), Column::new(100));
        assert_ne!(Column::new(1), Column::new(100));
    }
}
