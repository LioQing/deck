/// Peekable iterator that can peek at the next range of items.
#[derive(Debug)]
pub struct NextRangePeek<Iter>
where
    Iter: Iterator,
{
    iter: Iter,
    peeked: Vec<Iter::Item>,
}

impl<I> NextRangePeek<I>
where
    I: Iterator,
{
    /// Create a new peekable iterator.
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            peeked: Vec::new(),
        }
    }

    /// Peek the next range of items.
    pub fn peek(&mut self, n: usize) -> &[I::Item] {
        while self.peeked.len() < n {
            if let Some(item) = self.iter.next() {
                self.peeked.push(item);
            } else {
                break;
            }
        }

        &self.peeked[..std::cmp::min(n, self.peeked.len())]
    }

    /// Peek while the condition is true.
    pub fn peek_while<F>(&mut self, mut f: F) -> &[I::Item]
    where
        F: FnMut(&I::Item) -> bool,
    {
        let mut matched_count = self.peeked.iter().take_while(|x| f(x)).count();
        if matched_count < self.peeked.len() {
            return &self.peeked[..matched_count];
        }

        while let Some(item) = self.iter.next() {
            if f(&item) {
                self.peeked.push(item);
                matched_count += 1;
            } else {
                self.peeked.push(item);
                break;
            }
        }

        &self.peeked[..std::cmp::min(matched_count, self.peeked.len())]
    }
}

impl<Iter> Clone for NextRangePeek<Iter>
where
    Iter: Iterator + Clone,
    Vec<<Iter as Iterator>::Item>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
            peeked: self.peeked.clone(),
        }
    }
}

impl<Iter> Iterator for NextRangePeek<Iter>
where
    Iter: Iterator,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.peeked.is_empty() {
            self.iter.next()
        } else {
            Some(self.peeked.remove(0))
        }
    }
}

/// Extension for peeking at the next range of items.
pub trait NextRangePeekExt: Iterator {
    /// Peek the next range of items.
    fn next_range_peek(self) -> NextRangePeek<Self>
    where
        Self: Sized;
}

impl<Iter> NextRangePeekExt for Iter
where
    Iter: Iterator,
{
    fn next_range_peek(self) -> NextRangePeek<Self> {
        NextRangePeek::new(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_range_peek_peek() {
        let input = vec![1, 2, 3, 4, 5];
        let mut peek = input.iter().next_range_peek();
        assert_eq!(peek.peek(1), &[&1]);
        assert_eq!(peek.peek(2), &[&1, &2]);
        assert_eq!(peek.peek(3), &[&1, &2, &3]);
        assert_eq!(peek.peek(4), &[&1, &2, &3, &4]);
        assert_eq!(peek.peek(5), &[&1, &2, &3, &4, &5]);
        assert_eq!(peek.peek(6), &[&1, &2, &3, &4, &5]);
        assert_eq!(peek.next(), Some(&1));
        assert_eq!(peek.next(), Some(&2));
        assert_eq!(peek.next(), Some(&3));
        assert_eq!(peek.next(), Some(&4));
        assert_eq!(peek.next(), Some(&5));
        assert_eq!(peek.next(), None);
    }

    #[test]
    fn test_next_range_peek_peek_while() {
        let input = vec![1, 2, 3, 4, 5];
        let mut peek = input.iter().next_range_peek();
        assert_eq!(peek.peek_while(|x| *x < &3), &[&1, &2]);
        assert_eq!(peek.next(), Some(&1));
        assert_eq!(peek.next(), Some(&2));
        assert_eq!(peek.next(), Some(&3));
        assert_eq!(peek.next(), Some(&4));
        assert_eq!(peek.next(), Some(&5));
        assert_eq!(peek.next(), None);
    }
}
