use crate::{Column, RangeOrCell, A1};

#[allow(clippy::from_over_into)]
impl Into<RangeOrCell> for Column {
    fn into(self) -> RangeOrCell {
        RangeOrCell::ColumnRange {
            from: self,
            to: self,
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<A1> for Column {
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
    fn into_column() {
        assert_eq!(
            RangeOrCell::ColumnRange {
                from: Column::new(0),
                to: Column::new(0),
            },
            Column::new(0).into()
        );
    }

    #[test]
    fn into_a1() {
        assert_eq!(
            A1 {
                sheet_name: None,
                reference: RangeOrCell::ColumnRange {
                    from: Column::new(0),
                    to: Column::new(0),
                },
            },
            Column::new(0).into()
        );
    }
}
