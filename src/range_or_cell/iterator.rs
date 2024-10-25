use super::RangeOrCell;
use crate::{Address, Column, Row};
use std::iter;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum HorizontalDirection {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VerticalDirection {
    Down,
    Up,
}

/// Each `RangeOrCell` requires a different strategy of iteration, so the underlying iterators
/// reflect that by having an enum variant for each corresponding iterator.
#[derive(Debug, Clone)]
pub enum RangeOrCellIterator {
    /// Just stores and emits a single `Address`
    Cell { address: Option<Address> },

    /// Iterates from one column to another, one-by-one.
    ColumnRange {
        current: Option<Column>,
        horizontal_direction: HorizontalDirection,
        end: Column,
    },

    /// For each of the non-contiguous regions, call their iterator function until it's empty.
    /// Basically act as an aggregation of iterators.
    NonContiguous {
        iter: Option<Box<RangeOrCellIterator>>,
        range_or_cells: Vec<RangeOrCell>,
        i: usize,
    },

    /// Go row-by-row, left to right until current matches end
    Range {
        current: Option<Address>,
        end: Address,
        horizontal_direction: HorizontalDirection,
        start: Address,
        vertical_direction: VerticalDirection,
    },

    /// Iterate one-by-one from the start to end row.
    RowRange {
        current: Option<Row>,
        end: Row,
        vertical_direction: VerticalDirection,
    },
}

fn horizontal_direction<C: AsRef<Column>>(a: C, b: C) -> HorizontalDirection {
    if a.as_ref() < b.as_ref() {
        HorizontalDirection::Right
    } else {
        HorizontalDirection::Left
    }
}

fn vertical_direction<R: AsRef<Row>>(a: R, b: R) -> VerticalDirection {
    if a.as_ref() < b.as_ref() {
        VerticalDirection::Down
    } else {
        VerticalDirection::Up
    }
}

impl RangeOrCell {
    pub fn iter(&self) -> RangeOrCellIterator {
        match self {
            RangeOrCell::Cell(a) => RangeOrCellIterator::Cell { address: Some(*a) },

            RangeOrCell::ColumnRange { from, to } => RangeOrCellIterator::ColumnRange {
                current: Some(*from),
                horizontal_direction: horizontal_direction(from, to),
                end: *to,
            },

            RangeOrCell::NonContiguous(range_or_cells) => RangeOrCellIterator::NonContiguous {
                iter: None,
                range_or_cells: range_or_cells.clone(),
                i: 0,
            },

            RangeOrCell::Range { from, to } => RangeOrCellIterator::Range {
                current: Some(*from),
                end: *to,
                horizontal_direction: horizontal_direction(from, to),
                start: *from,
                vertical_direction: vertical_direction(from, to),
            },

            RangeOrCell::RowRange { from, to } => RangeOrCellIterator::RowRange {
                current: Some(*from),
                end: *to,
                vertical_direction: vertical_direction(from, to),
            },
        }
    }
}

impl iter::Iterator for RangeOrCellIterator {
    type Item = RangeOrCell;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Cell { ref mut address } => {
                let a = (*address)?;
                *address = None;
                Some(a.into())
            }

            Self::ColumnRange {
                ref mut current,
                end,
                horizontal_direction,
            } => {
                let c = (*current)?;

                *current = if c == *end {
                    None
                } else if *horizontal_direction == HorizontalDirection::Right {
                    Some(c.shift_right(1))
                } else {
                    Some(c.shift_left(1))
                };

                Some(c.into())
            }

            Self::NonContiguous {
                ref mut i,
                ref mut iter,
                range_or_cells,
            } => {
                // if we have an active iter, just use it until it runs out
                if let Some(i) = iter {
                    let n = i.next();
                    if n.is_some() {
                        return n;
                    }
                }

                if let Some(r) = range_or_cells.get(*i) {
                    let mut r_iter = r.iter();
                    let next_value = r_iter.next();

                    // we have an iterator - save it and increment `i` to signify we'll move onto
                    // the next `RangeOrCell`
                    *i += 1;
                    *iter = Some(Box::new(r_iter));

                    next_value
                } else {
                    None
                }
            }

            Self::Range {
                ref mut current,
                horizontal_direction,
                start,
                end,
                vertical_direction,
            } => {
                let c = (*current)?;
                let current_col: &Column = c.as_ref();

                // figure out the next value by traversing left/right row-wise then up/down
                *current =
                    // if we're past `end` (depending on which direction) then we're done
                    if c == *end {
                        None
                    // are we hitting the bounds of the range?
                    } else if current_col == end.as_ref() {
                        // then we need to shift up or down *and* reset our position on the next row
                        Some(if *vertical_direction == VerticalDirection::Up {
                            c.shift_up(1)
                        } else {
                            c.shift_down(1)
                        }.with_x(start.column.x))
                    } else {
                        // we're in-bounds so we just need to shift left or right
                        Some(if *horizontal_direction == HorizontalDirection::Left {
                            c.shift_left(1)
                        } else {
                            c.shift_right(1)
                        })
                    };

                Some(c.into())
            }

            Self::RowRange {
                ref mut current,
                end,
                vertical_direction,
            } => {
                let c = (*current)?;

                *current = if c == *end {
                    None
                } else if *vertical_direction == VerticalDirection::Down {
                    Some(c.shift_down(1))
                } else {
                    Some(c.shift_up(1))
                };

                Some(c.into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn range_to_strs(range: RangeOrCell) -> Vec<String> {
        range.iter().map(|r| r.to_string()).collect()
    }

    #[test]
    fn iter_cell() {
        let range = RangeOrCell::Cell((0, 0).into());

        assert_eq!(range_to_strs(range), vec!["A1"]);
    }

    #[test]
    fn iter_column_range() {
        let range = RangeOrCell::ColumnRange {
            from: 0.into(),
            to: 5.into(),
        };

        assert_eq!(
            range_to_strs(range),
            vec!["A:A", "B:B", "C:C", "D:D", "E:E", "F:F"]
        );
    }

    #[test]
    fn iter_column_range_backwards() {
        let range = RangeOrCell::ColumnRange {
            from: 5.into(),
            to: 0.into(),
        };

        assert_eq!(
            range_to_strs(range),
            vec!["F:F", "E:E", "D:D", "C:C", "B:B", "A:A"]
        );
    }

    #[test]
    fn iter_non_contiguous() {
        let range = RangeOrCell::NonContiguous(vec![
            RangeOrCell::Cell((0, 0).into()),
            RangeOrCell::ColumnRange {
                from: 1.into(),
                to: 2.into(),
            },
            RangeOrCell::Cell((2, 2).into()),
        ]);

        assert_eq!(range_to_strs(range), vec!["A1", "B:B", "C:C", "C3"]);
    }

    #[test]
    fn iter_range() {
        let range = RangeOrCell::Range {
            from: (0, 0).into(),
            to: (3, 3).into(),
        };

        assert_eq!(
            range_to_strs(range),
            vec![
                "A1", "B1", "C1", "D1", "A2", "B2", "C2", "D2", "A3", "B3", "C3", "D3", "A4", "B4",
                "C4", "D4",
            ]
        );
    }

    #[test]
    fn iter_range_backwards() {
        let range = RangeOrCell::Range {
            from: (3, 3).into(),
            to: (0, 0).into(),
        };

        assert_eq!(
            range_to_strs(range),
            vec![
                "D4", "C4", "B4", "A4", "D3", "C3", "B3", "A3", "D2", "C2", "B2", "A2", "D1", "C1",
                "B1", "A1",
            ]
        );
    }

    #[test]
    fn iter_row_range() {
        let range = RangeOrCell::RowRange {
            from: 0.into(),
            to: 5.into(),
        };

        assert_eq!(
            range_to_strs(range),
            vec!["1:1", "2:2", "3:3", "4:4", "5:5", "6:6"]
        );
    }

    #[test]
    fn iter_row_range_backwards() {
        let range = RangeOrCell::RowRange {
            from: 5.into(),
            to: 0.into(),
        };

        assert_eq!(
            range_to_strs(range),
            vec!["6:6", "5:5", "4:4", "3:3", "2:2", "1:1"]
        );
    }

    #[test]
    fn iter_row_range_single() {
        let range = RangeOrCell::RowRange {
            from: 0.into(),
            to: 0.into(),
        };

        assert_eq!(range_to_strs(range), vec!["1:1"]);
    }
}
