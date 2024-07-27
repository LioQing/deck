use super::*;

/// Span of source code.
/// Used to store location of tokens and errors in source code.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Span {
    pub start: SrcCodePoint,
    pub len: usize,
}

/// A wrapper to store any value with span information.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Span,
}
