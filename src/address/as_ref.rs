use crate::{Address, Column, Row};

impl AsRef<Column> for Address {
    fn as_ref(&self) -> &Column {
        &self.column
    }
}

impl AsRef<Row> for Address {
    fn as_ref(&self) -> &Row {
        &self.row
    }
}
