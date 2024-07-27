/// Code location in source code.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct SrcCodePoint {
    pub ch: char,
    pub line: usize,
    pub col: usize,
    pub idx: usize,
}
