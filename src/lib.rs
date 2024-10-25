//! # a1
//!
//! A library for parsing to and from A1 spreadsheet notation. A1 notation uses letters A-Z for
//! columns and a one-based number for the row.  So for example at position `(0, 0)` of a spreadsheet
//! (the top left corner) is "A1".  `(1, 1)` is "B2", `(1, 0)` is "B1", etc.  
//!
//! ## Instantiating `A1`s
//!
//! The most common need is to parse a string:
//!
//! ```
//! use a1::{A1, Address, Column, RangeOrCell, Row};
//! use std::str::FromStr;
//!
//! let a1 = A1::from_str("A1").unwrap();
//! // it parses it into an instance:
//! assert_eq!(a1,
//!     A1 {
//!         sheet_name: None,
//!         reference: RangeOrCell::Cell(Address {
//!             column: Column { absolute: false, x: 0 },
//!             row: Row { absolute: false, y: 0 },
//!         }),
//!     });
//! // and can display it back:
//! assert_eq!(&a1.to_string(), "A1");
//!
//! // you can also just call `a1::new`:
//! let from_col_a_to_d = a1::new("Foo!A:D").unwrap();
//! assert_eq!(from_col_a_to_d,
//!     A1 {
//!         sheet_name: Some("Foo".to_string()),
//!         reference: RangeOrCell::ColumnRange {
//!             from: Column { absolute: false, x: 0 },
//!             to: Column { absolute: false, x: 3 },
//!         },
//!     });
//!
//! assert_eq!(&from_col_a_to_d.to_string(), "Foo!A:D");
//! ```
//!
//! If you have zero-based coordinates and want to represent them as A1, there are several `fn`s
//! for instantiating:
//!
//! ```
//! # use a1::*;
//! // to create a reference to a specific cell:
//! assert_eq!(&a1::cell(2, 2).to_string(), "C3");
//!
//! // a reference to an entire column
//! assert_eq!(&a1::column(5).to_string(), "F:F");
//!
//! // or an entire row
//! assert_eq!(&a1::row(5).to_string(), "6:6");
//!
//! // and finally a range between two cells:
//! assert_eq!(&a1::range((0, 0), (4, 4)).to_string(), "A1:E5");
//! ```
//!
//! ## Contains
//!
//! Given all the various combinations or cells, ranges, row ranges, column ranges and
//! non-contiguous ranges you can calculate if one reference contains another.
//!
//! ```
//! # use a1::*;
//! // a column contains any cell in that column:
//! let col_a = a1::new("A:A").unwrap();
//! let a1 = a1::new("A1").unwrap();
//! assert!(col_a.contains(&a1));
//!
//! // likewise, a row range contains anything between it:
//! let top_5_rows = a1::new("1:5").unwrap();
//! let b2 = a1::new("B2").unwrap();
//! assert!(top_5_rows.contains(&b2));
//!
//! // and a range between two points works as you'd expect (it forms a rectangle)
//! let c3_to_j20 = a1::new("C3:J20").unwrap();
//! let d5 = a1::new("D5").unwrap();
//! assert!(c3_to_j20.contains(&d5));
//! ```
//!
//! ## Into/From/AsRef impls
//!
//! As much as possible it implements `Into`/`From` and `AsRef` to convert between the various
//! structs.  Generally you can go from more specific to less specific but not the other way
//! around.  You typically should work with `A1` structs but you can also use these traits to work
//! with these lower level ones and cast them upwards.
//!
//! ```
//! # use a1::*;
//! // an address can act as a column or row using AsRef:
//! let a1 = Address::new(0, 0);
//! assert_eq!(&Column::new(0), a1.as_ref());
//! assert_eq!(&Row::new(0), a1.as_ref());
//!
//! // addresses, columns and rows can `into()` "upwards" to an A1 or RangeOrCell
//! let col_b = Column::new(1);
//! assert_eq!(
//!     RangeOrCell::ColumnRange {
//!         from: Column::new(1),
//!         to: Column::new(1),
//!     },
//!     col_b.into());
//!
//! assert_eq!(
//!     A1 {
//!         sheet_name: None,
//!         reference: RangeOrCell::ColumnRange {
//!             from: Column::new(1),
//!             to: Column::new(1),
//!         },
//!     },
//!     col_b.into());
//! ```
//!
//! ## Shifting
//!
//! You can move references (and ranges) around:
//!
//! ```
//! # use a1::*;
//! // A1 -> D1 -> D3 -> C3
//! assert_eq!(
//!     &a1::cell(0, 0)
//!         .shift_right(3)
//!         .shift_down(2)
//!         .shift_left(1)
//!         .to_string(),
//!     "C3");
//! ```
//!
//! ## Iterators
//!
//! You can iterate through the various types of ranges.
//!
//! ```
//! # use crate::*;
//! // a cell just emits itself (once)
//! assert_eq!(
//!     a1::cell(0, 0)
//!         .iter().map(|r| r.to_string()).collect::<Vec<_>>(),
//!     vec!["A1"]);
//!
//! // a column range iterates column-wise
//! assert_eq!(
//!     a1::new("D:G").unwrap()
//!         .iter().map(|r| r.to_string()).collect::<Vec<_>>(),
//!     vec!["D:D", "E:E", "F:F", "G:G"]);
//!
//! // and a row range goes row-wise
//! assert_eq!(
//!     a1::new("3:6").unwrap()
//!         .iter().map(|r| r.to_string()).collect::<Vec<_>>(),
//!     vec!["3:3", "4:4", "5:5", "6:6"]);
//!
//! // a grid-based range goes row-by-row
//! assert_eq!(
//!     a1::new("A1:C3").unwrap()
//!         .iter().map(|r| r.to_string()).collect::<Vec<_>>(),
//!     vec![
//!         "A1", "B1", "C1",
//!         "A2", "B2", "C2",
//!         "A3", "B3", "C3",
//!     ]);
//! ```
//!
//! ### A1 Reference Examples
//!
//! Here is a table illustrating A1 references:
//!
//! | **Reference**   | **Meaning**               |
//! |:----------------|:--------------------------|
//! | `"A1"`          | Cell A1                   |
//! | `"A1:B5"`       | Cells A1 through B5       |
//! | `"C5:D9,G9:H16"`| A multiple-area selection |
//! | `"A:A"`         | Column A                  |
//! | `"1:1"`         | Row 1                     |
//! | `"A:C"`         | Columns A through C       |
//! | `"1:5"`         | Rows 1 through 5          |
//! | `"1:1,3:3,8:8"` | Rows 1, 3, and 8          |
//! | `"A:A,C:C,F:F"` | Columns A, C, and F       |
//!
//
// TODO:
//
// * implement `IntoIterator` for `RangeOrCell`
//
use std::str::FromStr;

mod a1;
mod address;
mod column;
mod error;
mod range_or_cell;
mod row;

pub use a1::A1;
pub use address::Address;
pub use column::Column;
pub use error::Error;
pub use range_or_cell::RangeOrCell;
pub use row::Row;

pub type Result<T> = std::result::Result<T, Error>;

pub type Index = usize;

pub(crate) static ALPHA: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

/// Create an `A1` referencing a cell at a given address (x/y)
pub fn cell(x: Index, y: Index) -> A1 {
    A1 {
        sheet_name: None,
        reference: RangeOrCell::Cell(Address::new(x, y)),
    }
}

/// Parse an A1-style string into an `A1`
pub fn new(s: &str) -> Result<A1> {
    A1::from_str(s)
}

/// Create a range between two cells
pub fn range<A: Into<Address>>(from: A, to: A) -> A1 {
    A1 {
        sheet_name: None,
        reference: RangeOrCell::Range {
            from: from.into(),
            to: to.into(),
        },
    }
}

/// An entire column
pub fn column<C: Into<Column>>(x: C) -> A1 {
    A1 {
        sheet_name: None,
        reference: RangeOrCell::column(x),
    }
}

/// A range between two columns
pub fn column_range<R: Into<Column>>(xa: R, xb: R) -> A1 {
    A1 {
        sheet_name: None,
        reference: RangeOrCell::column_range(xa, xb),
    }
}

/// An entire row
pub fn row<R: Into<Row>>(y: R) -> A1 {
    A1 {
        sheet_name: None,
        reference: RangeOrCell::row(y),
    }
}

/// A range between two rows
pub fn row_range<R: Into<Row>>(ya: R, yb: R) -> A1 {
    A1 {
        sheet_name: None,
        reference: RangeOrCell::row_range(ya, yb),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell() {
        let a1 = crate::cell(2, 3);

        assert_eq!(a1.sheet_name, None);
        assert_eq!(a1.reference, RangeOrCell::Cell(Address::new(2, 3)));
    }

    #[test]
    fn new() {
        let a1 = crate::new("Foo!A1").unwrap();

        assert_eq!(a1.sheet_name, Some("Foo".to_string()));
        assert_eq!(a1.reference, RangeOrCell::Cell(Address::new(0, 0)));
    }

    #[test]
    fn range() {
        let a1 = crate::range((0, 0), (22, 33));

        assert_eq!(a1.sheet_name, None);
        assert_eq!(
            a1.reference,
            RangeOrCell::Range {
                from: Address::new(0, 0),
                to: Address::new(22, 33),
            }
        );
    }

    #[test]
    fn column() {
        let a1 = crate::column(22);

        assert_eq!(a1.sheet_name, None);
        assert_eq!(
            a1.reference,
            RangeOrCell::ColumnRange {
                from: Column::new(22),
                to: Column::new(22),
            }
        );
    }

    #[test]
    fn column_range() {
        let a1 = crate::column_range(1, 420);

        assert_eq!(a1.sheet_name, None);
        assert_eq!(
            a1.reference,
            RangeOrCell::ColumnRange {
                from: Column::new(1),
                to: Column::new(420),
            }
        );
    }

    #[test]
    fn row() {
        let a1 = crate::row(11);

        assert_eq!(a1.sheet_name, None);
        assert_eq!(
            a1.reference,
            RangeOrCell::RowRange {
                from: Row::new(11),
                to: Row::new(11),
            }
        );
    }

    #[test]
    fn row_range() {
        let a1 = crate::row_range(0, 42);

        assert_eq!(a1.sheet_name, None);
        assert_eq!(
            a1.reference,
            RangeOrCell::RowRange {
                from: Row::new(0),
                to: Row::new(42),
            }
        );
    }
}
