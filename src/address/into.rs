//! We allow converting from a more specific type (Position) to a more general one (A1) but it
//! can't happen the other way around, so therefore we need to implement `Into` rather than
//! `From`
use crate::{Address, Column, RangeOrCell, Row, A1};

#[allow(clippy::from_over_into)]
impl Into<A1> for Address {
    fn into(self) -> A1 {
        A1 {
            sheet_name: None,
            reference: self.into(),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<Column> for Address {
    fn into(self) -> Column {
        self.column
    }
}

#[allow(clippy::from_over_into)]
impl Into<Row> for Address {
    fn into(self) -> Row {
        self.row
    }
}

#[allow(clippy::from_over_into)]
impl Into<RangeOrCell> for Address {
    fn into(self) -> RangeOrCell {
        RangeOrCell::Cell(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_into_a1() {
        let a1: A1 = Address::new(1, 2).into();
        assert_eq!(
            a1,
            A1 {
                sheet_name: None,
                reference: RangeOrCell::Cell(Address::new(1, 2)),
            }
        );
    }

    #[test]
    fn address_into_column() {
        let c: Column = Address::new(1, 2).into();
        assert_eq!(c, Column::new(1));
    }

    #[test]
    fn address_into_row() {
        let r: Row = Address::new(1, 2).into();
        assert_eq!(r, Row::new(2));
    }

    #[test]
    fn address_into_range_or_cell() {
        let r: RangeOrCell = Address::new(1, 2).into();
        assert_eq!(r, RangeOrCell::Cell(Address::new(1, 2)));
    }
}
