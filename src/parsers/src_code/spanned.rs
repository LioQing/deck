use super::*;

/// Span position in source code.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct SpanPos {
    pub line: usize,
    pub col: usize,
    pub idx: usize,
}

/// Span of source code.
/// Used to store location of tokens and errors in source code.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Span {
    pub start: SpanPos,
    pub len: usize,
}

impl Span {
    /// Create a new span.
    pub fn new(start: SpanPos, len: usize) -> Self {
        Self { start, len }
    }
}

/// A wrapper to store any value with span information.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    /// Create a new spanned value.
    pub fn new(value: T, span: Span) -> Self {
        Self { value, span }
    }

    /// Map the value.
    pub fn map<U, F>(self, f: F) -> Spanned<U>
    where
        F: FnOnce(T) -> U,
    {
        Spanned {
            value: f(self.value),
            span: self.span,
        }
    }
}
