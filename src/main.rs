use gat_lending_iterator::LendingIterator;

// The extension trait for normal Iterator.
trait IteratorExt<T> {
    fn filter_count(self, f: impl Fn(&T) -> bool) -> usize;
}
// The blanket implementation for normal Iterator.
impl<T, I: Iterator<Item = T>> IteratorExt<T> for I {
    fn filter_count(self, f: impl Fn(&T) -> bool) -> usize {
        self.filter(f).count()
    }
}

// The attempted extension trait for LendingIterator
pub trait LendingIteratorExt<T> {
    fn filter_count<P>(self, f: P) -> usize
    where
        P: FnMut(&T) -> bool;
}
// The attempted blanket implementation for normal LendingIterator.
impl<'a, I: LendingIterator + Sized> LendingIteratorExt<I::Item<'a>> for I {
    fn filter_count<P>(self, f: P) -> usize
    where
        P: FnMut(&I::Item<'a>) -> bool,
    {
        self.filter(f).count();
    }
}

struct LendingIter(usize);
impl LendingIterator for LendingIter {
    type Item<'a> = &'a usize
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        if self.0 > 9 {
            None
        } else {
            self.0 += 1;
            Some(&self.0)
        }
    }
}

// Example usage
fn main() {
    let arr = [1, 5, 3, 4, 6, 7, 2];
    assert_eq!(arr.iter().filter_count(|n| **n > 4), 3);

    let iter = LendingIter(0);
    //let count = iter.filter(|n| **n > 5).count();
    let count = iter.filter_count(|n| **n > 5);
    assert_eq!(count, 5);
}
