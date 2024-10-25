use super::RangeOrCell;
use std::fmt;

impl fmt::Display for RangeOrCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Cell(p) => write!(f, "{p}"),
            Self::ColumnRange { from, to } => write!(f, "{from}:{to}"),
            Self::NonContiguous(range_or_cells) => {
                let joined_range_or_cells = range_or_cells
                    .iter()
                    .map(|r| r.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                write!(f, "{joined_range_or_cells}")
            }
            Self::Range { from, to } => write!(f, "{from}:{to}"),
            Self::RowRange { from, to } => write!(f, "{from}:{to}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn display_cell() {
        assert_eq!(RangeOrCell::Cell(Address::new(0, 0)).to_string(), "A1");

        assert_eq!(
            RangeOrCell::Cell(Address::new(100, 100)).to_string(),
            "CW101"
        );
    }

    #[test]
    fn display_column_range() {
        assert_eq!(
            RangeOrCell::ColumnRange {
                from: Column::new(0),
                to: Column::new(10)
            }
            .to_string(),
            "A:K"
        );

        assert_eq!(
            RangeOrCell::ColumnRange {
                from: Column::new(5),
                to: Column::new(5)
            }
            .to_string(),
            "F:F"
        );
    }

    #[test]
    fn display_non_contiguous() {
        assert_eq!(
            RangeOrCell::NonContiguous(vec![
                RangeOrCell::Cell(Address::new(0, 0)),
                RangeOrCell::ColumnRange {
                    from: Column::new(0),
                    to: Column::new(10)
                },
                RangeOrCell::Range {
                    from: Address::new(0, 0),
                    to: Address::new(10, 10)
                },
            ])
            .to_string(),
            "A1, A:K, A1:K11"
        );
    }

    #[test]
    fn display_range() {
        assert_eq!(
            RangeOrCell::Range {
                from: Address::new(0, 0),
                to: Address::new(10, 10)
            }
            .to_string(),
            "A1:K11"
        );
    }

    #[test]
    fn display_row_range() {
        assert_eq!(
            RangeOrCell::RowRange {
                from: Row::new(0),
                to: Row::new(10)
            }
            .to_string(),
            "1:11"
        );

        assert_eq!(
            RangeOrCell::RowRange {
                from: Row::new(5),
                to: Row::new(5)
            }
            .to_string(),
            "6:6"
        );
    }
}
