//! # RangeOrCell
//!
//! Parsing and displaying a cell value (which can pretty much always be either a cell or a range).
//!
use crate::{Address, Column, Index, Row, A1};

mod display;
mod from_str;
mod into;
mod into_iterator;
pub mod iterator;

#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
#[cfg_attr(
    feature = "rkyv",
    rkyv(serialize_bounds(
    __S: rkyv::ser::Writer + rkyv::ser::Allocator,
    __S::Error: rkyv::rancor::Source,
)))]
#[cfg_attr(
    feature = "rkyv",
    rkyv(deserialize_bounds(__D::Error: rkyv::rancor::Source))
)]
#[cfg_attr(feature = "rkyv", rkyv(bytecheck(
    bounds(__C: rkyv::validation::ArchiveContext)
)))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum RangeOrCell {
    /// Just a single cell
    Cell(Address),

    /// A range between two columns
    ///
    /// * `from` - Where the range begins
    /// * `to` - Where the range ends
    ColumnRange { from: Column, to: Column },

    /// A set of cells and ranges
    ///
    /// Note: `rkyv` requires that we add the `omit_bounds` for anything self-referential.
    NonContiguous(#[cfg_attr(feature = "rkyv", rkyv(omit_bounds))] Vec<Self>),

    /// A range between two positions
    ///
    /// * `from` - Where the range begins
    /// * `to` - Where the range ends
    Range { from: Address, to: Address },

    /// A range between two rows
    ///
    /// * `from` - Where the range begins
    /// * `to` - Where the range ends
    RowRange { from: Row, to: Row },
}

impl RangeOrCell {
    /// Create a `RangeOrCell::ColumnRange` at the given `x` index
    pub fn column<C: Into<Column>>(x: C) -> Self {
        // Column -> RangeOrCell::ColumnRange
        x.into().into()
    }

    /// Create a `RangeOrCell::ColumnRange` between two columns.
    pub fn column_range<C: Into<Column>>(xa: C, xb: C) -> Self {
        Self::ColumnRange {
            from: xa.into(),
            to: xb.into(),
        }
    }

    /// Create a `RangeOrCell::Range` between two addresses.
    pub fn range<A: Into<Address>>(aa: A, ab: A) -> Self {
        Self::Range {
            from: aa.into(),
            to: ab.into(),
        }
    }

    /// Create a `RangeOrCell::RowRange` at the given `y` index
    pub fn row<R: Into<Row>>(y: R) -> Self {
        // Row -> RangeOrCell::RowRange
        y.into().into()
    }

    /// Create a `RangeOrCell::RowRange` between two rows.
    pub fn row_range<R: Into<Row>>(ya: R, yb: R) -> Self {
        Self::RowRange {
            from: ya.into(),
            to: yb.into(),
        }
    }

    /// This function has a lot going on because we need to handle every combination of every
    /// `RangeOrCell` containing every other combination of a `RangeOrCell`.  The rules are
    /// nuanced but I think intuitive if you think through how it would look on a grid.
    pub fn contains(&self, other: &Self) -> bool {
        match self {
            Self::Cell(a) => {
                match other {
                    // a cell can only contain itself, nothing larger (i.e. a range)
                    Self::Cell(other_a) => a == other_a,

                    // they're asking if `self` (a `Cell(Address)`) "contains" a list of ranges...
                    // the only way that would happen is if it was a list of the same point.  but
                    // for thoroughness we'll try:
                    Self::NonContiguous(o) => o.iter().all(|oa| oa.contains(self)),

                    // anything else is larger than a cell and wouldn't be able to be contained by
                    // it
                    _ => false,
                }
            }

            Self::ColumnRange { from, to } => {
                match other {
                    // it needs to be completely contained by the `self`s column range
                    Self::ColumnRange {
                        from: other_from,
                        to: other_to,
                    } => other_from.is_between(from, to) && other_to.is_between(from, to),

                    // the cell just needs to be between the two columns
                    Self::Cell(a) => {
                        let a_col: &Column = a.as_ref();
                        a_col.is_between(from, to)
                    }

                    // our column range has to contain all of `other`s points
                    Self::NonContiguous(r) => r.iter().all(|oa| oa.contains(self)),

                    Self::Range {
                        from: other_from,
                        to: other_to,
                    } => {
                        let other_from_col: &Column = other_from.as_ref();
                        let other_to_col: &Column = other_to.as_ref();

                        other_from_col.is_between(from, to) && other_to_col.is_between(from, to)
                    }

                    // a row range can never be contained inside a column range
                    Self::RowRange { .. } => false,
                }
            }

            // we just need to know if any of the ranges in our NonContiguous contain it
            Self::NonContiguous(range_or_cells) => range_or_cells.iter().any(|r| r.contains(other)),

            Self::RowRange { from, to } => {
                match other {
                    // a column range would never be able to be contained in a row range
                    Self::ColumnRange { .. } => false,

                    // the cell just needs to be between the two rows
                    Self::Cell(a) => {
                        let a_row: &Row = a.as_ref();
                        a_row.is_between(from, to)
                    }

                    Self::RowRange {
                        from: other_from,
                        to: other_to,
                    } => other_from.is_between(from, to) && other_to.is_between(from, to),

                    // our row range has to contain all of `other`s points
                    Self::NonContiguous(r) => r.iter().all(|oa| oa.contains(self)),

                    Self::Range {
                        from: other_from,
                        to: other_to,
                    } => {
                        let other_from_row: &Row = other_from.as_ref();
                        let other_to_row: &Row = other_to.as_ref();

                        other_from_row.is_between(from, to) && other_to_row.is_between(from, to)
                    }
                }
            }

            Self::Range { from, to } => {
                match other {
                    // a bounded range (`Range`) can't contain an unbounded range
                    Self::ColumnRange { .. } => false,

                    Self::Cell(a) => a.is_between(from, to),

                    // a bounded range (`Range`) can't contain an unbounded range
                    Self::RowRange { .. } => false,

                    // our row range has to contain all of `other`s points
                    Self::NonContiguous(r) => r.iter().all(|oa| oa.contains(self)),

                    Self::Range {
                        from: other_from,
                        to: other_to,
                    } => other_from.is_between(from, to) && other_to.is_between(from, to),
                }
            }
        }
    }

    pub fn shift_down(self, rows: usize) -> Self {
        match self {
            Self::Cell(a) => Self::Cell(a.shift_down(rows)),

            // column ranges don't really do anything by shifting down
            Self::ColumnRange { .. } => self,

            Self::NonContiguous(range_or_cells) => Self::NonContiguous(
                range_or_cells
                    .into_iter()
                    .map(|r| r.shift_down(rows))
                    .collect(),
            ),

            Self::Range { from, to } => Self::Range {
                from: from.shift_down(rows),
                to: to.shift_down(rows),
            },

            Self::RowRange { from, to } => Self::RowRange {
                from: from.shift_down(rows),
                to: to.shift_down(rows),
            },
        }
    }

    pub fn shift_left(self, columns: usize) -> Self {
        match self {
            Self::Cell(a) => Self::Cell(a.shift_left(columns)),

            Self::ColumnRange { to, from } => Self::ColumnRange {
                from: from.shift_left(columns),
                to: to.shift_left(columns),
            },

            Self::NonContiguous(range_or_cells) => Self::NonContiguous(
                range_or_cells
                    .into_iter()
                    .map(|r| r.shift_left(columns))
                    .collect(),
            ),

            Self::Range { from, to } => Self::Range {
                from: from.shift_left(columns),
                to: to.shift_left(columns),
            },

            // row ranges don't really do anything by shifting left
            Self::RowRange { .. } => self,
        }
    }

    pub fn shift_right(self, columns: usize) -> Self {
        match self {
            Self::Cell(a) => Self::Cell(a.shift_right(columns)),

            Self::ColumnRange { to, from } => Self::ColumnRange {
                from: from.shift_right(columns),
                to: to.shift_right(columns),
            },

            Self::NonContiguous(range_or_cells) => Self::NonContiguous(
                range_or_cells
                    .into_iter()
                    .map(|r| r.shift_right(columns))
                    .collect(),
            ),

            Self::Range { from, to } => Self::Range {
                from: from.shift_right(columns),
                to: to.shift_right(columns),
            },

            // row ranges don't do anything by shifting left
            Self::RowRange { .. } => self,
        }
    }

    pub fn shift_up(self, rows: usize) -> Self {
        match self {
            Self::Cell(a) => Self::Cell(a.shift_up(rows)),

            // column ranges don't do anything by shifting up
            Self::ColumnRange { .. } => self,

            Self::NonContiguous(range_or_cells) => Self::NonContiguous(
                range_or_cells
                    .into_iter()
                    .map(|r| r.shift_up(rows))
                    .collect(),
            ),

            Self::Range { from, to } => Self::Range {
                from: from.shift_up(rows),
                to: to.shift_up(rows),
            },

            Self::RowRange { from, to } => Self::RowRange {
                from: from.shift_up(rows),
                to: to.shift_up(rows),
            },
        }
    }

    /// Set the `x` component of the underlying `RangeOrCell`.  Depending on the variant of the
    /// enum the rules will be different
    pub fn with_x(self, x: Index) -> Self {
        match self {
            Self::Cell(a) => Self::Cell(a.with_x(x)),

            // a column range with a different x just becomes that range.  For example B:D being
            // set to C will just become C:C
            Self::ColumnRange { from, to } => Self::ColumnRange {
                from: from.with_x(x),
                to: to.with_x(x),
            },

            Self::NonContiguous(range_or_cells) => {
                Self::NonContiguous(range_or_cells.into_iter().map(|r| r.with_x(x)).collect())
            }

            Self::Range { from, to } => Self::Range {
                from: from.with_x(x),
                to: to.with_x(x),
            },

            // a row range has a y and we're setting an x.  which means we ended up with an X/Y
            // which is just a normal `Range`
            Self::RowRange { from, to } => Self::Range {
                from: Address::new(x, from.y),
                to: Address::new(x, to.y),
            },
        }
    }

    pub fn with_y(&self, y: usize) -> Self {
        match self {
            Self::Cell(a) => Self::Cell(a.with_y(y)),

            // a column range has an x and we're setting an y.  which means we ended up with an X/Y
            // which is just a normal `Range`
            Self::ColumnRange { from, to } => Self::Range {
                from: Address::new(from.x, y),
                to: Address::new(to.x, y),
            },

            Self::NonContiguous(range_or_cells) => {
                Self::NonContiguous(range_or_cells.iter().map(|r| r.with_y(y)).collect())
            }

            Self::Range { from, to } => Self::Range {
                from: from.with_y(y),
                to: to.with_y(y),
            },

            Self::RowRange { from, to } => Self::RowRange {
                from: from.with_y(y),
                to: to.with_y(y),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn contains_cell() {
        assert!(RangeOrCell::Cell((0, 0).into()).contains(&RangeOrCell::Cell((0, 0).into())));

        assert!(
            !RangeOrCell::Cell((0, 0).into()).contains(&RangeOrCell::Range {
                from: (0, 0).into(),
                to: (10, 10).into()
            })
        );
    }

    #[test]
    fn contains_column_range() {
        let col_range = RangeOrCell::ColumnRange {
            from: 0.into(),
            to: 5.into(),
        };

        assert!(col_range.contains(&RangeOrCell::Cell((1, 1).into())));
        assert!(col_range.contains(&RangeOrCell::Cell((0, 0).into())));
        assert!(col_range.contains(&RangeOrCell::ColumnRange {
            from: 1.into(),
            to: 1.into()
        }));

        // the contained region is too big (overlaps it)
        assert!(!col_range.contains(&RangeOrCell::Range {
            from: (0, 0).into(),
            to: (10, 10).into()
        }));
        assert!(!col_range.contains(&RangeOrCell::RowRange {
            from: 0.into(),
            to: 10.into()
        }));
        assert!(!col_range.contains(&RangeOrCell::Cell((100, 100).into())));
    }

    #[test]
    fn contains_range() {
        let range = RangeOrCell::Range {
            from: (0, 0).into(),
            to: (5, 5).into(),
        };

        assert!(range.contains(&RangeOrCell::Cell((1, 1).into())));
        assert!(range.contains(&RangeOrCell::Cell((0, 0).into())));

        // the contained region is too big (overlaps it)
        assert!(!range.contains(&RangeOrCell::Range {
            from: (0, 0).into(),
            to: (10, 10).into()
        }));
        assert!(!range.contains(&RangeOrCell::RowRange {
            from: 1.into(),
            to: 1.into()
        }));
        assert!(!range.contains(&RangeOrCell::ColumnRange {
            from: 0.into(),
            to: 10.into()
        }));
        assert!(!range.contains(&RangeOrCell::Cell((100, 100).into())));
    }

    #[test]
    fn contains_row_range() {
        let row_range = RangeOrCell::RowRange {
            from: 0.into(),
            to: 5.into(),
        };

        assert!(row_range.contains(&RangeOrCell::Cell((1, 1).into())));
        assert!(row_range.contains(&RangeOrCell::Cell((0, 0).into())));
        assert!(row_range.contains(&RangeOrCell::RowRange {
            from: 1.into(),
            to: 1.into()
        }));

        // the contained region is too big (overlaps it)
        assert!(!row_range.contains(&RangeOrCell::Range {
            from: (0, 0).into(),
            to: (10, 10).into()
        }));
        assert!(!row_range.contains(&RangeOrCell::ColumnRange {
            from: 0.into(),
            to: 10.into()
        }));
        assert!(!row_range.contains(&RangeOrCell::Cell((100, 100).into())));
    }

    #[test]
    fn shift_down_cell() {
        assert_eq!(
            RangeOrCell::Cell((10, 10).into()).shift_down(5),
            RangeOrCell::Cell((10, 15).into())
        );
    }

    #[test]
    fn shift_down_non_contiguous() {
        assert_eq!(
            RangeOrCell::NonContiguous(vec![
                RangeOrCell::ColumnRange {
                    from: 5.into(),
                    to: 10.into()
                },
                RangeOrCell::Range {
                    from: (5, 5).into(),
                    to: (10, 10).into()
                },
            ])
            .shift_down(55),
            RangeOrCell::NonContiguous(vec![
                RangeOrCell::ColumnRange {
                    from: 5.into(),
                    to: 10.into()
                },
                RangeOrCell::Range {
                    from: (5, 60).into(),
                    to: (10, 65).into()
                },
            ])
        );
    }

    #[test]
    fn shift_down_range() {
        assert_eq!(
            RangeOrCell::Range {
                from: (0, 0).into(),
                to: (5, 5).into()
            }
            .shift_down(5),
            RangeOrCell::Range {
                from: (0, 5).into(),
                to: (5, 10).into()
            }
        );
    }

    #[test]
    fn shift_down_row_range() {
        assert_eq!(
            RangeOrCell::RowRange {
                from: 0.into(),
                to: 5.into()
            }
            .shift_down(5),
            RangeOrCell::RowRange {
                from: 5.into(),
                to: 10.into()
            }
        );
    }

    #[test]
    fn shift_left_cell() {
        assert_eq!(
            RangeOrCell::Cell((10, 10).into()).shift_left(5),
            RangeOrCell::Cell((5, 10).into())
        );
    }

    #[test]
    fn shift_left_range() {
        assert_eq!(
            RangeOrCell::Range {
                from: (10, 10).into(),
                to: (15, 15).into()
            }
            .shift_left(5),
            RangeOrCell::Range {
                from: (5, 10).into(),
                to: (10, 15).into()
            }
        );
    }

    #[test]
    fn shift_left_column_range() {
        assert_eq!(
            RangeOrCell::ColumnRange {
                from: 10.into(),
                to: 15.into()
            }
            .shift_left(5),
            RangeOrCell::ColumnRange {
                from: 5.into(),
                to: 10.into()
            }
        );
    }

    #[test]
    fn shift_right_cell() {
        assert_eq!(
            RangeOrCell::Cell((10, 10).into()).shift_right(5),
            RangeOrCell::Cell((15, 10).into())
        );
    }

    #[test]
    fn shift_right_range() {
        assert_eq!(
            RangeOrCell::Range {
                from: (10, 10).into(),
                to: (15, 15).into()
            }
            .shift_right(5),
            RangeOrCell::Range {
                from: (15, 10).into(),
                to: (20, 15).into()
            }
        );
    }

    #[test]
    fn shift_right_column_range() {
        assert_eq!(
            RangeOrCell::ColumnRange {
                from: 10.into(),
                to: 15.into()
            }
            .shift_right(5),
            RangeOrCell::ColumnRange {
                from: 15.into(),
                to: 20.into()
            }
        );
    }

    #[test]
    fn shift_up_cell() {
        assert_eq!(
            RangeOrCell::Cell((10, 10).into()).shift_up(5),
            RangeOrCell::Cell((10, 5).into())
        );
    }

    #[test]
    fn shift_up_range() {
        assert_eq!(
            RangeOrCell::Range {
                from: (50, 50).into(),
                to: (55, 55).into()
            }
            .shift_up(5),
            RangeOrCell::Range {
                from: (50, 45).into(),
                to: (55, 50).into()
            }
        );
    }

    #[test]
    fn shift_up_row_range() {
        assert_eq!(
            RangeOrCell::RowRange {
                from: 25.into(),
                to: 50.into()
            }
            .shift_up(5),
            RangeOrCell::RowRange {
                from: 20.into(),
                to: 45.into()
            }
        );
    }

    #[test]
    fn with_x_cell() {
        assert_eq!(
            RangeOrCell::Cell((10, 10).into()).with_x(5),
            RangeOrCell::Cell((5, 10).into())
        );
    }

    #[test]
    fn with_x_column_range() {
        assert_eq!(
            RangeOrCell::ColumnRange {
                from: 5.into(),
                to: 10.into()
            }
            .with_x(55),
            RangeOrCell::ColumnRange {
                from: 55.into(),
                to: 55.into()
            }
        );
    }

    #[test]
    fn with_x_non_contiguous() {
        assert_eq!(
            RangeOrCell::NonContiguous(vec![
                RangeOrCell::ColumnRange {
                    from: 5.into(),
                    to: 10.into()
                },
                RangeOrCell::Range {
                    from: (5, 5).into(),
                    to: (10, 10).into()
                },
            ])
            .with_x(55),
            RangeOrCell::NonContiguous(vec![
                RangeOrCell::ColumnRange {
                    from: 55.into(),
                    to: 55.into()
                },
                RangeOrCell::Range {
                    from: (55, 5).into(),
                    to: (55, 10).into()
                },
            ])
        );
    }

    #[test]
    fn with_x_range() {
        assert_eq!(
            RangeOrCell::Range {
                from: (5, 5).into(),
                to: (10, 10).into()
            }
            .with_x(55),
            RangeOrCell::Range {
                from: (55, 5).into(),
                to: (55, 10).into()
            }
        );
    }

    #[test]
    fn with_x_row_range() {
        assert_eq!(
            RangeOrCell::RowRange {
                from: 5.into(),
                to: 10.into()
            }
            .with_x(55),
            RangeOrCell::Range {
                from: (55, 5).into(),
                to: (55, 10).into()
            }
        );
    }

    #[test]
    fn with_y_cell() {
        assert_eq!(
            RangeOrCell::Cell((10, 10).into()).with_y(5),
            RangeOrCell::Cell((10, 5).into())
        );
    }

    #[test]
    fn with_y_column_range() {
        assert_eq!(
            RangeOrCell::ColumnRange {
                from: 5.into(),
                to: 10.into()
            }
            .with_y(55),
            RangeOrCell::Range {
                from: (5, 55).into(),
                to: (10, 55).into()
            }
        );
    }

    #[test]
    fn with_y_non_contiguous() {
        assert_eq!(
            RangeOrCell::NonContiguous(vec![
                RangeOrCell::ColumnRange {
                    from: 5.into(),
                    to: 10.into()
                },
                RangeOrCell::Range {
                    from: (5, 5).into(),
                    to: (10, 10).into()
                },
            ])
            .with_y(55),
            RangeOrCell::NonContiguous(vec![
                RangeOrCell::Range {
                    from: (5, 55).into(),
                    to: (10, 55).into()
                },
                RangeOrCell::Range {
                    from: (5, 55).into(),
                    to: (10, 55).into()
                },
            ])
        );
    }

    #[test]
    fn with_y_range() {
        assert_eq!(
            RangeOrCell::Range {
                from: (5, 5).into(),
                to: (10, 10).into()
            }
            .with_y(55),
            RangeOrCell::Range {
                from: (5, 55).into(),
                to: (10, 55).into()
            }
        );
    }

    #[test]
    fn with_y_row_range() {
        assert_eq!(
            RangeOrCell::RowRange {
                from: 5.into(),
                to: 10.into()
            }
            .with_y(55),
            RangeOrCell::RowRange {
                from: 55.into(),
                to: 55.into()
            }
        );
    }
}
