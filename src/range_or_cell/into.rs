use super::{RangeOrCell, A1};

/// We allow converting from a more specific type (RangeOrCell) to a more general one (A1) but
/// it can't happen the other way around, so therefore we need to implement `Into` rather than
/// `From`
#[allow(clippy::from_over_into)]
impl Into<A1> for RangeOrCell {
    fn into(self) -> A1 {
        A1 {
            sheet_name: None,
            reference: self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Address, A1};

    #[test]
    fn into() {
        let range_or_cell = RangeOrCell::Cell(Address::new(0, 0));
        let a1: A1 = range_or_cell.clone().into();

        assert_eq!(a1.sheet_name, None);
        assert_eq!(a1.reference, range_or_cell);
    }
}
