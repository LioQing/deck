use super::*;
use crate::{utils::AdvanceIterExt, EvalDebugOption, Evaluator, SynNode, SynNodeKind};

/// Semantic parser result.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct SemParserResult<Item> {
    pub item: Item,
    pub advance: usize,
}

/// Semantic parser.
#[derive(Debug, Clone)]
pub struct SemParser<Iter>
where
    Iter: Iterator<Item = SynNode> + std::fmt::Debug + Clone + AdvanceIterExt,
{
    iter: Iter,
}

impl<Iter> SemParser<Iter>
where
    Iter: Iterator<Item = SynNode> + std::fmt::Debug + Clone + AdvanceIterExt,
{
    /// Create a new semantic parser.
    pub fn new(iter: Iter) -> Self {
        Self { iter }
    }

    /// Evaluate.
    pub fn evaluate(self) {
        let nodes = self.collect::<Vec<_>>();
        Evaluator::new(nodes.iter()).for_each(drop);
    }

    /// Evaluate with debug options.
    pub fn evaluate_with_debug(self, debug_options: EvalDebugOption) {
        let nodes = self.collect::<Vec<_>>();
        Evaluator::new_with_debug(nodes.iter(), debug_options).for_each(drop);
    }

    /// Parse an expression.
    pub fn parse_expr(&self) -> Option<SemParserResult<SemNodeExpr>> {
        let mut iter = self.iter.clone();
        let node = iter.next()?;
        match &node.value {
            SynNodeKind::Ident(idents) => {
                let idents = idents.clone();
                Some(SemParserResult {
                    item: SemNodeExpr {
                        value: SemNodeExprKind::Ident(idents),
                        span: node.span,
                    },
                    advance: 1,
                })
            }
            SynNodeKind::Brac {
                open: '(',
                close: ')',
                children,
            } => {
                let children = children.clone();
                let parser = SemParser::new(children.into_iter());
                let exprs = parser.parse_expr_vec()?;
                Some(SemParserResult {
                    item: SemNodeExpr {
                        value: SemNodeExprKind::Inner(exprs.item),
                        span: node.span,
                    },
                    advance: 1,
                })
            }
            SynNodeKind::Error { msg, children } => {
                let children = children.clone();
                let parser = SemParser::new(children.into_iter());
                let expr = parser.parse_expr();
                Some(SemParserResult {
                    item: SemNodeExpr {
                        value: SemNodeExprKind::Error {
                            msg: msg.clone(),
                            children: expr.map_or(vec![], |x| vec![x.item]),
                        },
                        span: node.span,
                    },
                    advance: 1,
                })
            }
            _ => None,
        }
    }

    /// Parse a vec of expression.
    pub fn parse_expr_vec(&self) -> Option<SemParserResult<Vec<SemNodeExpr>>> {
        let mut advance = 0;
        let mut iter = self.iter.clone();
        let mut exprs = vec![];
        while let Some(expr) = SemParser::new(iter.clone()).parse_expr() {
            exprs.push(expr.item);
            iter.advance(expr.advance);
            advance += expr.advance;
        }
        Some(SemParserResult {
            item: exprs,
            advance,
        })
    }

    /// Parse a definition.
    pub fn parse_def(&self) -> Option<SemParserResult<SemNode>> {
        let mut advance = 0;
        let mut iter = self.iter.clone();
        let idents = SemParser::new(iter.clone()).parse_expr_vec()?;
        iter.advance(idents.advance);
        advance += idents.advance;

        match iter.next()? {
            SynNode {
                value:
                    SynNodeKind::Brac {
                        open: '{',
                        close: '}',
                        children,
                    },
                span,
            } => {
                let mut children_iter = children.into_iter();
                let body = SemParser::new(children_iter.clone()).parse_def_vec()?;
                children_iter.advance(body.advance);
                let expr = SemParser::new(children_iter.clone()).parse_expr_vec()?;
                children_iter.advance(expr.advance);
                Some(SemParserResult {
                    item: SemNode {
                        value: SemNodeKind::Def {
                            idents: idents.item,
                            body: body.item,
                            exprs: expr.item,
                        },
                        span,
                    },
                    advance: advance + 1,
                })
            }
            SynNode {
                value: SynNodeKind::Error { msg, children },
                span,
            } => {
                let children = children.clone();
                let parser = SemParser::new(children.into_iter());
                let def = parser.parse_def();
                Some(SemParserResult {
                    item: SemNode {
                        value: SemNodeKind::Error {
                            msg: msg.clone(),
                            children: def.map_or(vec![], |x| vec![x.item]),
                        },
                        span,
                    },
                    advance: advance + 1,
                })
            }
            _ => None,
        }
    }

    /// Parse a vec of definition.
    pub fn parse_def_vec(&self) -> Option<SemParserResult<Vec<SemNode>>> {
        let mut advance = 0;
        let mut iter = self.iter.clone();
        let mut defs = vec![];
        while let Some(def) = SemParser::new(iter.clone()).parse_def() {
            defs.push(def.item);
            iter.advance(def.advance);
            advance += def.advance;
        }
        Some(SemParserResult {
            item: defs,
            advance,
        })
    }
}

impl<Iter> Iterator for SemParser<Iter>
where
    Iter: Iterator<Item = SynNode> + std::fmt::Debug + Clone + AdvanceIterExt,
{
    type Item = SemNode;

    fn next(&mut self) -> Option<Self::Item> {
        let def = self.parse_def()?;
        self.iter.advance(def.advance);
        Some(def.item)
    }
}
