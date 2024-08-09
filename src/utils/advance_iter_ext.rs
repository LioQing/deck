/// Extension for advancing an iterator by `n` steps.
pub trait AdvanceIterExt: Iterator {
    /// Advance the iterator by `n` steps.
    fn advance(&mut self, n: usize);
}

impl<Iter> AdvanceIterExt for Iter
where
    Iter: Iterator,
{
    fn advance(&mut self, n: usize) {
        for _ in 0..n {
            self.next();
        }
    }
}
