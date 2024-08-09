use crate::parsers::SemNodeExpr;
use crate::{EvalIdents, SemNode};

/// Definition value.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum EvalDefValue<'a> {
    /// Base: a base value
    Base,

    /// Reference: a reference to a definition
    Ref(EvalIdents),

    /// Expanded: an expanded definition
    Expanded(EvalIdents),

    /// Node: a semantic node
    Node {
        body: &'a Vec<SemNode>,
        exprs: &'a Vec<SemNodeExpr>,
    },
}
