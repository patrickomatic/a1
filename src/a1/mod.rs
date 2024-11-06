//! # A1
//!
//! `A1` is the most encompassing and generic of the structs included in this package - it can be
//! any kind of range (cell, range, column range, etc) also includes the sheet name if set.  It can
//! perform all of the most general operations like shifting up/down/left/right and `contains`
//! set operation against another `A1`.  
//!
//! Unless you need the specificity of another type you should prefer to write code that operates
//! in terms of `A1`s.  Also when parsing `str`s, this crate will generally return results in
//! terms of `A1`s rather than the other types.
//!
//! ### Links
//!
//! * [Google Sheets API Overview](https://developers.google.com/sheets/api/guides/concepts)
//! * [Refer to Cells and Ranges by Using A1 Notation](https://learn.microsoft.com/en-us/office/vba/excel/concepts/cells-and-ranges/refer-to-cells-and-ranges-by-using-a1-notation)
//!
use crate::RangeOrCell;

mod display;
mod from_str;
mod into_iterator;
mod iterator;

#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct A1 {
    pub sheet_name: Option<String>,
    pub reference: RangeOrCell,
}

impl A1 {
    /// Is `other` completely contained within `self`?  They also must be in the same sheet
    /// (meaning `self.sheet_name` == `other.sheet_name`).
    pub fn contains(&self, other: &Self) -> bool {
        self.sheet_name == other.sheet_name && self.reference.contains(&other.reference)
    }

    /// Returns a new `A1` shifted downwards by `rows` rows.
    pub fn shift_down(self, rows: usize) -> Self {
        Self {
            reference: self.reference.shift_down(rows),
            ..self
        }
    }

    /// Returns a new `A1` shifted left by `columns` columns.
    pub fn shift_left(self, columns: usize) -> Self {
        Self {
            reference: self.reference.shift_left(columns),
            ..self
        }
    }

    /// Returns a new `A1` shifted right by `columns` columns.
    pub fn shift_right(self, columns: usize) -> Self {
        Self {
            reference: self.reference.shift_right(columns),
            ..self
        }
    }

    /// Returns a new `A1` shifted up by `rows` rows.
    pub fn shift_up(self, rows: usize) -> Self {
        Self {
            reference: self.reference.shift_up(rows),
            ..self
        }
    }

    /// Clone into a new `A1` with the given `sheet_name`
    pub fn with_sheet_name<S: Into<String>>(self, sheet_name: S) -> Self {
        Self {
            sheet_name: Some(sheet_name.into()),
            ..self
        }
    }

    /// Return a new `A1` with the given X position set.  If the `reference` already has an `x`
    /// component, it will be overwritten in the returned value.
    pub fn with_x(self, x: usize) -> Self {
        Self {
            reference: self.reference.with_x(x),
            ..self
        }
    }

    /// Return a new `A1` with the given Y position set.  If the `reference` already has an `y`
    /// component, it will be overwritten in the returned value.
    pub fn with_y(self, y: usize) -> Self {
        Self {
            reference: self.reference.with_y(y),
            ..self
        }
    }

    pub fn without_sheet_name(self) -> Self {
        Self {
            sheet_name: None,
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn contains_different_name() {
        let a1_a = A1 {
            sheet_name: Some("Something".to_string()),
            reference: RangeOrCell::Cell((1, 1).into()),
        };
        let a1_b = A1 {
            sheet_name: Some("Something else".to_string()),
            reference: RangeOrCell::Cell((1, 1).into()),
        };

        assert!(!a1_a.contains(&a1_b));
    }

    #[test]
    fn contains_true() {
        let a1_a = A1 {
            sheet_name: None,
            reference: RangeOrCell::Cell((1, 1).into()),
        };
        let a1_b = A1 {
            sheet_name: None,
            reference: RangeOrCell::Cell((1, 1).into()),
        };

        assert!(a1_a.contains(&a1_b));
    }

    #[test]
    fn shift_down() {
        let a1 = A1 {
            sheet_name: Some("Test1".to_string()),
            reference: RangeOrCell::Cell((1, 1).into()),
        };

        assert_eq!("Test1!B3", a1.shift_down(1).to_string());
    }

    #[test]
    fn shift_left() {
        let a1 = A1 {
            sheet_name: Some("Test1".to_string()),
            reference: RangeOrCell::Cell((1, 1).into()),
        };

        assert_eq!("Test1!A2", a1.shift_left(1).to_string());
    }

    #[test]
    fn shift_right() {
        let a1 = A1 {
            sheet_name: None,
            reference: RangeOrCell::Cell((1, 1).into()),
        };

        assert_eq!("C2", a1.shift_right(1).to_string());
    }

    #[test]
    fn shift_up() {
        let a1 = A1 {
            sheet_name: None,
            reference: RangeOrCell::Cell((1, 1).into()),
        };

        assert_eq!("B1", a1.shift_up(1).to_string());
    }

    #[test]
    fn with_sheet_name() {
        let a1 = A1 {
            sheet_name: None,
            reference: RangeOrCell::Cell((1, 1).into()),
        };

        assert_eq!(
            Some("foo".to_string()),
            a1.with_sheet_name("foo").sheet_name
        );
    }

    #[test]
    fn with_x() {
        let a1 = A1 {
            sheet_name: None,
            reference: RangeOrCell::Cell((1, 1).into()),
        };

        assert_eq!("F2", a1.with_x(5).to_string());
    }

    #[test]
    fn with_y() {
        let a1 = A1 {
            sheet_name: None,
            reference: RangeOrCell::Cell((1, 1).into()),
        };

        assert_eq!("B22", a1.with_y(21).to_string());
    }

    #[test]
    fn without_sheet_name() {
        let a1 = A1 {
            sheet_name: Some("foo".to_string()),
            reference: RangeOrCell::Cell((1, 1).into()),
        };

        assert_eq!(None, a1.without_sheet_name().sheet_name);
    }
}
