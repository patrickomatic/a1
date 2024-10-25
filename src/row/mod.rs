//! # Row
use crate::Index;
use std::cmp;

mod as_ref;
mod display;
mod from;
mod from_str;
mod into;
mod ord;
mod partial_ord;

// need to implement this here in order to #[derive(Eq)] below
impl PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
        self.y == other.y
    }
}

#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Copy, Clone, Debug, Eq)]
pub struct Row {
    /// If the row was was specified with a `$`.
    pub absolute: bool,

    /// The zero-based index of the cell.  For A1 it's 0, A2 it's 1, A3 is 2, etc..
    pub y: Index,
}

impl Row {
    /// Does `self` contain the given row or address?  Either it's the same row or a point within
    /// the row
    pub fn contains<T: AsRef<Self>>(&self, other: T) -> bool {
        self.y == other.as_ref().y
    }

    /// Is `self` (inclusively) between the given `a` and `b` rows
    pub fn is_between<T: AsRef<Self>>(&self, a: T, b: T) -> bool {
        let a_ref = a.as_ref();
        let b_ref = b.as_ref();
        let from = cmp::min(a_ref, b_ref);
        let to = cmp::max(a_ref, b_ref);

        self >= from && self <= to
    }

    /// Create a new `Row` (with `absolute`: `false`)
    pub fn new(y: Index) -> Self {
        Self { absolute: false, y }
    }

    /// Shift the row down by the given amount.
    pub fn shift_down(&self, rows: Index) -> Self {
        if rows == 0 {
            return *self;
        }

        Self {
            y: self.y.saturating_add(rows),
            ..*self
        }
    }

    /// Shift the row up by the given amount.
    pub fn shift_up(&self, rows: Index) -> Self {
        if rows == 0 {
            return *self;
        }

        Self {
            y: std::cmp::max(self.y.saturating_sub(rows), 0),
            ..*self
        }
    }

    /// Set the `y` and return a `Copy`ed `Row`
    pub fn with_y(&self, y: Index) -> Self {
        Self { y, ..*self }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn contains_false() {
        assert!(!Row::new(0).contains(Row::new(1)));
        assert!(!Row::new(0).contains(Address::new(1, 1)));
        assert!(!Row::new(0).contains(Address::new(100, 10)));
    }

    #[test]
    fn contains_true() {
        assert!(Row::new(0).contains(Row::new(0)));
        assert!(Row::new(0).contains(Address::new(0, 0)));
        assert!(Row::new(0).contains(Address::new(10, 0)));
    }

    #[test]
    fn is_between_true() {
        assert!(Row::new(5).is_between(&Row::new(0), &Row::new(20)));
        assert!(Row::new(5).is_between(&Row::new(5), &Row::new(20)));
    }

    #[test]
    fn is_between_false() {
        assert!(!Row::new(5).is_between(&Row::new(0), &Row::new(2)));
    }

    #[test]
    fn shift_down() {
        assert_eq!(Row::new(0).shift_down(1), Row::new(1));
        assert_eq!(Row::new(100).shift_down(1000), Row::new(1100));
    }

    #[test]
    fn shift_up() {
        assert_eq!(Row::new(10).shift_up(5), Row::new(5));
        assert_eq!(Row::new(0).shift_up(10), Row::new(0));
        assert_eq!(Row::new(100).shift_up(0), Row::new(100));
    }
}
