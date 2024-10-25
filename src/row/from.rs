use crate::{Index, Row};

impl From<Index> for Row {
    fn from(index: Index) -> Self {
        Row::new(index)
    }
}
