use crate::Spanned;

/// Token kinds.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum TokenKind {
    /** Open bracket */
    OpenBrac(char),

    /** Close bracket */
    CloseBrac(char),

    /** Identifier */
    Ident(String),

    /** Spaces */
    Spaces,

    /** Newline */
    Newlines,
}

/// Token.
pub type Token = Spanned<TokenKind>;
