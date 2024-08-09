use crate::Spanned;
use strum_macros::EnumIs;

/// Syntactic node kind.
#[derive(Debug, PartialEq, Eq, Clone, Hash, EnumIs)]
pub enum SynNodeKind {
    /// Identifier: an identifiers
    Ident(String),

    /// Brackets: a node enclosed in brackets
    Brac(BracSynNode),

    /// Error: an error node
    Err(ErrSynNode),
}

/// Brackets syntactic node.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct BracSynNode {
    /// Open bracket.
    pub open: char,

    /// Close bracket.
    pub close: char,

    /// Children nodes.
    pub children: Vec<SynNode>,
}

/// Error syntactic node.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ErrSynNode {
    /// Error message.
    pub msg: String,

    /// Children nodes.
    pub children: Vec<SynNode>,
}

/// Syntactic node.
pub type SynNode = Spanned<SynNodeKind>;

impl SynNode {
    /// Create a simple display string.
    pub fn simple_display(&self) -> String {
        fn indent(lines: String) -> String {
            lines
                .lines()
                .map(|line| format!("    {}\n", line))
                .collect::<String>()
        }

        match &self.value {
            SynNodeKind::Ident(ident) => {
                format!("Ident '{}'", ident)
            }
            SynNodeKind::Brac(BracSynNode {
                open,
                close,
                children,
            }) => match &children[..] {
                [] => format!("Brac '{open}{close}' []"),
                nodes => format!(
                    "Brac '{open}{close}' [\n{}]",
                    nodes
                        .iter()
                        .map(|x| format!("{},\n", x.simple_display()))
                        .map(indent)
                        .collect::<String>()
                ),
            },
            SynNodeKind::Err(ErrSynNode { msg, children }) => match &children[..] {
                [] => format!("Error '{msg}' []"),
                nodes => format!(
                    "Error '{msg}' [\n{}]",
                    nodes
                        .iter()
                        .map(|x| format!("{},\n", x.simple_display()))
                        .map(indent)
                        .collect::<String>()
                ),
            },
        }
    }
}
