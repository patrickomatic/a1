use super::iterator::RangeOrCellIterator;
use super::RangeOrCell;

pub struct RangeOrCellIntoIterator {
    inner_iter: RangeOrCellIterator,
}

impl Iterator for RangeOrCellIntoIterator {
    type Item = RangeOrCell;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter.next()
    }
}

impl IntoIterator for RangeOrCell {
    type Item = RangeOrCell;
    type IntoIter = RangeOrCellIntoIterator;

    fn into_iter(self) -> RangeOrCellIntoIterator {
        RangeOrCellIntoIterator {
            inner_iter: self.iter(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn into_iter_column_range() {
        assert_eq!(
            RangeOrCell::column_range(0, 5)
                .into_iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>(),
            vec!["A:A", "B:B", "C:C", "D:D", "E:E", "F:F"]
        );
    }
}
