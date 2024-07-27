pub trait AdvanceIter {
    /// Advance the iterator by `n` steps.
    fn advance(&mut self, n: usize);
}

impl<Iter> AdvanceIter for Iter
where
    Iter: Iterator,
{
    fn advance(&mut self, n: usize) {
        for _ in 0..n {
            self.next();
        }
    }
}

pub trait SimpleDisplay {
    /// Create a simple display string.
    fn simple_display(&self) -> String;
}
