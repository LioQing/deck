use crate::utils::SimpleDisplay;
use crate::Spanned;
use strum_macros::EnumIs;

/// Token kinds.
#[derive(Debug, PartialEq, Eq, Clone, Hash, EnumIs)]
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

impl SimpleDisplay for Token {
    fn simple_display(&self) -> String {
        format!("{:?}", self.value)
    }
}
