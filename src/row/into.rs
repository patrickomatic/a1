use crate::{RangeOrCell, Row, A1};

#[allow(clippy::from_over_into)]
impl Into<RangeOrCell> for Row {
    fn into(self) -> RangeOrCell {
        RangeOrCell::RowRange {
            from: self,
            to: self,
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<A1> for Row {
    fn into(self) -> A1 {
        A1 {
            sheet_name: None,
            reference: self.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn into_row() {
        assert_eq!(
            RangeOrCell::RowRange {
                from: Row::new(0),
                to: Row::new(0),
            },
            Row::new(0).into()
        );
    }

    #[test]
    fn into_a1() {
        assert_eq!(
            A1 {
                sheet_name: None,
                reference: RangeOrCell::RowRange {
                    from: Row::new(0),
                    to: Row::new(0),
                },
            },
            Row::new(0).into()
        );
    }
}
