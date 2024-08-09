use super::*;
use crate::utils::NextRangePeekExt;
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
        Lexer::new(self.next_range_peek())
    }
}

impl<'a> Iterator for SrcCodeIter<'a> {
    type Item = SrcCode;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((idx, ch)) => {
                let pos = SpanPos {
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

                Some(SrcCode {
                    value: ch,
                    span: Span::new(pos, 1),
                })
            }
            None => None,
        }
    }
}

/// Source code iterator extension.
pub trait SrcCodeIterExt<'a> {
    fn src_code(self) -> SrcCodeIter<'a>;
}

impl<'a> SrcCodeIterExt<'a> for std::str::CharIndices<'a> {
    fn src_code(self) -> SrcCodeIter<'a> {
        SrcCodeIter::new(self)
    }
}
