//! # Column
use crate::Index;
use std::cmp;

mod as_ref;
mod display;
mod from;
mod from_str;
mod into;
mod ord;
mod partial_eq;
mod partial_ord;

#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Copy, Clone, Debug, Eq)]
pub struct Column {
    pub absolute: bool,
    pub x: Index,
}

impl Column {
    /// Is `self` (inclusively) between the given `a` and `b` columns
    pub fn is_between<T: AsRef<Self>>(&self, a: T, b: T) -> bool {
        let a_ref = a.as_ref();
        let b_ref = b.as_ref();
        let from = cmp::min(a_ref, b_ref);
        let to = cmp::max(a_ref, b_ref);

        self >= from && self <= to
    }

    /// Does `self` contain the given column or address?  Either it's the same column or a point
    /// within the column
    pub fn contains<T: AsRef<Self>>(&self, other: T) -> bool {
        self.x == other.as_ref().x
    }

    pub fn new(x: Index) -> Self {
        Self { absolute: false, x }
    }

    /// Shift the column left by the given amount.
    pub fn shift_left(&self, columns: Index) -> Self {
        Self {
            // make sure we don't shift negative
            x: std::cmp::max(self.x.saturating_sub(columns), 0),
            ..*self
        }
    }

    /// Shift the column right by the given amount.
    pub fn shift_right(&self, columns: Index) -> Self {
        Self {
            // make sure we don't shift past max(usize)
            x: self.x.saturating_add(columns),
            ..*self
        }
    }

    /// Set the `x` and return a `Copy`ed `Column`
    pub fn with_x(&self, x: Index) -> Self {
        Self { x, ..*self }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn contains_true() {
        assert!(Column::new(5).contains(Column::new(5)));
        assert!(Column::new(5).contains(Address::new(5, 10)));
    }

    #[test]
    fn contains_false() {
        assert!(!Column::new(5).contains(Column::new(50)));
        assert!(!Column::new(5).contains(Address::new(50, 10)));
    }

    #[test]
    fn is_between_true() {
        assert!(Column::new(5).is_between(&Column::new(0), &Column::new(20)));
        assert!(Column::new(5).is_between(&Column::new(5), &Column::new(20)));
    }

    #[test]
    fn is_between_false() {
        assert!(!Column::new(5).is_between(&Column::new(0), &Column::new(2)));
    }

    #[test]
    fn shift_left() {
        assert_eq!(Column::new(5).shift_left(3), Column::new(2));
        assert_eq!(Column::new(5).shift_left(300), Column::new(0));
    }

    #[test]
    fn shift_right() {
        assert_eq!(Column::new(5).shift_right(3), Column::new(8));
    }
}
