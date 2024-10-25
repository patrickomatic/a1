use crate::{Address, Column, Error, RangeOrCell, Result, Row};
use std::str::FromStr;

fn parse_str(a1: &str) -> Result<RangeOrCell> {
    if let Some((l, r)) = a1.split_once(':') {
        if l.chars().all(|c| c == '$' || c.is_ascii_digit())
            && r.chars().all(|c| c == '$' || c.is_ascii_digit())
        {
            Ok(RangeOrCell::RowRange {
                from: Row::from_str(l)?,
                to: Row::from_str(r)?,
            })
        } else if l.chars().all(|c| c == '$' || c.is_ascii_alphabetic())
            && r.chars().all(|c| c == '$' || c.is_ascii_alphabetic())
        {
            Ok(RangeOrCell::ColumnRange {
                from: Column::from_str(l)?,
                to: Column::from_str(r)?,
            })
        } else {
            Ok(RangeOrCell::Range {
                from: Address::from_str(l)?,
                to: Address::from_str(r)?,
            })
        }
    } else {
        Ok(RangeOrCell::Cell(Address::from_str(a1)?))
    }
}

impl FromStr for RangeOrCell {
    type Err = Error;

    fn from_str(a1: &str) -> Result<Self> {
        let range_strs: Vec<&str> = a1.split(',').collect();

        let count = range_strs.len();
        if count > 1 {
            let mut ranges = vec![];
            for range_str in range_strs {
                ranges.push(parse_str(range_str)?);
            }

            Ok(RangeOrCell::NonContiguous(ranges))
        } else if let Some(s) = range_strs.first() {
            parse_str(s)
        } else {
            Err(Error::parse_error(a1, "No valid A1 references found"))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::str::FromStr;

    #[test]
    fn from_str_cell() {
        assert_eq!(
            RangeOrCell::Cell(Address::new(0, 0)),
            RangeOrCell::from_str("A1").unwrap()
        );
    }

    #[test]
    fn from_str_non_contiguous() {
        assert_eq!(
            RangeOrCell::NonContiguous(vec![
                RangeOrCell::Cell(Address::new(0, 0)),
                RangeOrCell::Cell(Address::new(1, 1)),
                RangeOrCell::Range {
                    from: Address::new(0, 0),
                    to: Address::new(2, 2),
                },
            ]),
            RangeOrCell::from_str("A1,B2,A1:C3").unwrap()
        );
    }

    #[test]
    fn from_str_range() {
        assert_eq!(
            RangeOrCell::Range {
                from: Address::new(0, 0),
                to: Address::new(2, 2),
            },
            RangeOrCell::from_str("A1:C3").unwrap()
        );
    }

    #[test]
    fn from_str_row_range() {
        assert_eq!(
            RangeOrCell::RowRange {
                from: Row::new(0),
                to: Row::new(5),
            },
            RangeOrCell::from_str("1:6").unwrap()
        );
    }

    #[test]
    fn from_str_column_range() {
        assert_eq!(
            RangeOrCell::ColumnRange {
                from: Column::new(0),
                to: Column::new(2),
            },
            RangeOrCell::from_str("A:C").unwrap()
        );
    }
}
