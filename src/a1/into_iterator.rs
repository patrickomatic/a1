use super::iterator::A1Iterator;
use super::A1;

pub struct A1IntoIterator {
    inner_iter: A1Iterator,
}

impl Iterator for A1IntoIterator {
    type Item = A1;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter.next()
    }
}

impl IntoIterator for A1 {
    type Item = A1;
    type IntoIter = A1IntoIterator;

    fn into_iter(self) -> A1IntoIterator {
        A1IntoIterator {
            inner_iter: self.iter(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn into_iter_no_sheet_name() {
        assert_eq!(
            range((0, 0), (0, 4))
                .into_iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>(),
            vec!["A1", "A2", "A3", "A4", "A5"]
        );
    }

    #[test]
    fn into_iter_with_sheet_name() {
        assert_eq!(
            range((0, 0), (0, 4))
                .with_sheet_name("Foo")
                .into_iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>(),
            vec!["Foo!A1", "Foo!A2", "Foo!A3", "Foo!A4", "Foo!A5"]
        );
    }
}
