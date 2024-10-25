use crate::{Address, Index};

#[allow(clippy::from_over_into)]
impl From<(Index, Index)> for Address {
    fn from((column, row): (Index, Index)) -> Self {
        Address::new(column, row)
    }
}
