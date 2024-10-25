use super::Column;

impl AsRef<Column> for Column {
    fn as_ref(&self) -> &Column {
        self
    }
}
