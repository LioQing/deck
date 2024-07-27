use crate::Spanned;

/// Syntactic node kind.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum SynNodeKind {
    /// Identifier: an identifiers
    Ident(String),

    /// Brackets: a node enclosed in brackets
    Brac {
        open: char,
        close: char,
        children: Vec<SynNode>,
    },

    /// Error: an error node
    Error { msg: String, children: Vec<SynNode> },
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
            SynNodeKind::Brac {
                open,
                close,
                children,
            } => match &children[..] {
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
            SynNodeKind::Error { msg, children } => match &children[..] {
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
