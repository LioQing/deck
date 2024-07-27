use crate::{utils::SimpleDisplay, Spanned};

/// Semantic node kind.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum SemNodeKind {
    /// Definition: mapping from identifiers to an expression block
    Def {
        idents: Vec<SemNodeExpr>,
        body: Vec<SemNode>,
        exprs: Vec<SemNodeExpr>,
    },

    /// Error: an error occurred
    Error { msg: String, children: Vec<SemNode> },
}

/// Semantic node expression kind.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum SemNodeExprKind {
    /// Identifier: an identifier
    Ident(String),

    /// Inner: an inner expression block
    Inner(Vec<SemNodeExpr>),

    /// Error: an error occurred
    Error {
        msg: String,
        children: Vec<SemNodeExpr>,
    },
}

/// Semantic node expression.
pub type SemNodeExpr = Spanned<SemNodeExprKind>;

impl SimpleDisplay for SemNodeExpr {
    fn simple_display(&self) -> String {
        fn indent(lines: String) -> String {
            lines
                .lines()
                .map(|line| format!("    {}\n", line))
                .collect::<String>()
        }

        fn format_ident(ident: &str) -> String {
            format!("ident: {}", ident)
        }

        fn format_inner(inner: &Vec<SemNodeExpr>) -> String {
            format!(
                "inner: [\n{}]",
                inner
                    .iter()
                    .map(|x| format!("{},", x.simple_display()))
                    .map(indent)
                    .collect::<String>()
            )
        }

        fn format_error(msg: &str, children: &Vec<SemNodeExpr>) -> String {
            format!(
                "error: '{}', [\n{}]",
                msg,
                children
                    .iter()
                    .map(|x| format!("{},", x.simple_display()))
                    .map(indent)
                    .collect::<String>()
            )
        }

        match &self.value {
            SemNodeExprKind::Ident(ident) => format_ident(ident),
            SemNodeExprKind::Inner(inner) => format_inner(inner),
            SemNodeExprKind::Error { msg, children } => format_error(msg, children),
        }
    }
}

/// Semantic node.
pub type SemNode = Spanned<SemNodeKind>;

impl SimpleDisplay for SemNode {
    fn simple_display(&self) -> String {
        fn indent(lines: String) -> String {
            lines
                .lines()
                .map(|line| format!("    {}\n", line))
                .collect::<String>()
        }

        fn format_idents(idents: &[SemNodeExpr]) -> String {
            match idents {
                [] => "idents: []".to_string(),
                idents => format!(
                    "idents: [\n{}]",
                    idents
                        .iter()
                        .map(|x| format!("{},\n", x.simple_display()))
                        .map(indent)
                        .collect::<String>()
                ),
            }
        }

        fn format_body(body: &[SemNode]) -> String {
            match body {
                [] => "body: []".to_string(),
                body => format!(
                    "body: [\n{}]",
                    body.iter()
                        .map(|x| format!("{},\n", x.simple_display()))
                        .map(indent)
                        .collect::<String>()
                ),
            }
        }

        fn format_expr(expr: &[SemNodeExpr]) -> String {
            match expr {
                [] => "expr: []".to_string(),
                expr => format!(
                    "expr: [\n{}]",
                    expr.iter()
                        .map(|x| format!("{},\n", x.simple_display()))
                        .map(indent)
                        .collect::<String>()
                ),
            }
        }

        match &self.value {
            SemNodeKind::Def {
                idents,
                body,
                exprs: expr,
            } => {
                format!(
                    "Def {{\n{}}}",
                    indent(format!(
                        "{}\n{}\n{}\n",
                        format_idents(idents),
                        format_body(body),
                        format_expr(expr),
                    )),
                )
            }
            SemNodeKind::Error { msg, children } => {
                format!(
                    "Error {{\n{}}}",
                    indent(format!(
                        "msg: '{}',\nchildren: [\n{}]",
                        msg,
                        children
                            .iter()
                            .map(|x| format!("{},\n", x.simple_display()))
                            .map(indent)
                            .collect::<String>()
                    )),
                )
            }
        }
    }
}
