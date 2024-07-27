use super::*;
use crate::Lexer;

/// Source code iterator.
#[derive(Debug, Clone)]
pub struct SrcCodeIter<'a> {
    iter: std::str::CharIndices<'a>,
    curr_line: usize,
    curr_col: usize,
}

impl<'a> SrcCodeIter<'a> {
    /// Create a new source code iterator.
    pub fn new(iter: std::str::CharIndices<'a>) -> Self {
        Self {
            iter,
            curr_line: 1,
            curr_col: 1,
        }
    }

    /// Create a lexer.
    pub fn lexer(self) -> Lexer<Self> {
        Lexer::new(self)
    }
}

impl<'a> Iterator for SrcCodeIter<'a> {
    type Item = SrcCodePoint;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((idx, ch)) => {
                let info = SrcCodePoint {
                    ch,
                    line: self.curr_line,
                    col: self.curr_col,
                    idx,
                };

                if ch == '\n' {
                    self.curr_line += 1;
                    self.curr_col = 1;
                } else {
                    self.curr_col += 1;
                }

                Some(info)
            }
            None => None,
        }
    }
}

/// Source code iterator trait.
pub trait SrcCodeIterTrait<'a> {
    fn src_code_iter(&self) -> SrcCodeIter<'a>;
}

impl<'a> SrcCodeIterTrait<'a> for &'a str {
    fn src_code_iter(&self) -> SrcCodeIter<'a> {
        SrcCodeIter::new(self.char_indices())
    }
}
