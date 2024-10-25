use crate::{Column, Index};

impl From<Index> for Column {
    fn from(index: Index) -> Self {
        Column::new(index)
    }
}
