use std::collections::HashMap;
use crate::{
    AdvanceIter, EvalDefValue, EvalIdents, EvalIdentsExtensions, EvalIdentsKind, EvalStack,
    EvalStackResolveResult, SemNode, SemNodeExpr, SemNodeExprKind, SemNodeKind, SimpleDisplay,
};

/// Semantic node iterator traits.
pub trait AdvanceSemNodeIterator<'a>: Iterator<Item = &'a SemNode> + std::fmt::Debug + AdvanceIter {}
impl<'a, T: Iterator<Item = &'a SemNode> + std::fmt::Debug + AdvanceIter> AdvanceSemNodeIterator<'a> for T {}

/// Evaluate identifiers option.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum EvalIdentsIdentOption {
    ResolveWithStack,
    AlwaysExpr,
    AlwaysParam,
}

bitflags::bitflags! {
    /// Evaluate option.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct EvalDebugOption: u8 {
        const NONE = 0b00;
        const STACK = 0b01;
        const CALL = 0b10;
        const ALL = 0b11;
    }
}

/// Evaluator
#[derive(Debug)]
pub struct Evaluator<'a> {
    stack: EvalStack<'a>,
    debug_options: EvalDebugOption,
}

impl<'a> Evaluator<'a> {
    /// Create a new evaluator.
    pub fn new<Iter>(iter: Iter) -> Self where Iter: AdvanceSemNodeIterator<'a> + 'a {
        Self {
            stack: EvalStack::new(iter),
            debug_options: EvalDebugOption::NONE,
        }
    }

    /// Create a new evaluator with debug options.
    pub fn new_with_debug<Iter>(iter: Iter, debug_options: EvalDebugOption) -> Self
    where
        Iter: AdvanceSemNodeIterator<'a> + 'a
    {
        Self {
            stack: EvalStack::new(iter),
            debug_options,
        }
    }

    /// Evaluate identifiers.
    pub fn eval_idents(
        &mut self,
        exprs: &'a Vec<SemNodeExpr>,
        ident_option: EvalIdentsIdentOption,
    ) -> EvalIdents {
        let mut result = vec![];

        for expr in exprs {
            match expr {
                SemNodeExpr {
                    value: SemNodeExprKind::Ident(ident),
                    ..
                } => match ident_option {
                    EvalIdentsIdentOption::ResolveWithStack => {
                        let idents = vec![EvalIdentsKind::Expr(ident.clone())];

                        fn unwrap_idents(mut idents: Vec<EvalIdentsKind>) -> String {
                            match idents.pop().unwrap() {
                                EvalIdentsKind::Expr(ident) => ident,
                                _ => unreachable!(),
                            }
                        }

                        match self.eval_exprs(&idents, false) {
                            Some(EvalDefValue::Ref(_)) => result.push(EvalIdentsKind::Expr(unwrap_idents(idents))),
                            None => result.push(EvalIdentsKind::Param(unwrap_idents(idents))),
                            Some(value) => panic!("unexpected definition: {:?}", value), // Likely unreachable
                        }
                    }
                    EvalIdentsIdentOption::AlwaysExpr => result.push(EvalIdentsKind::Expr(ident.clone())),
                    EvalIdentsIdentOption::AlwaysParam => result.push(EvalIdentsKind::Param(ident.clone())),
                },
                SemNodeExpr {
                    value: SemNodeExprKind::Inner(inner),
                    ..
                } => {
                    let inner_idents = self.eval_idents(inner, ident_option);
                    result.push(EvalIdentsKind::Inner(inner_idents));
                }
                SemNodeExpr {
                    value: SemNodeExprKind::Error { msg, .. },
                    ..
                } => panic!("error: {}", msg),
            }
        }

        result
    }

    /// Evaluate expressions.
    ///
    /// Returns:
    /// - `Some(EvalDefValue::Base)`: if `idents` is empty
    /// - `Some(EvalDefValue::Ref(idents))`: if the expression is found
    /// - `Some(EvalDefValue::Expanded(idents))`: if the expression is expanded
    /// - `None`: if the expression is not found
    pub fn eval_exprs(&mut self, idents: &EvalIdents, debug: bool) -> Option<EvalDefValue<'a>> {
        if idents.is_empty() {
            return Some(EvalDefValue::Base);
        }

        let mut curr = idents;
        if debug {
            println!("-----------dbg-----------\n{}", curr.simple_display());
        }

        while let Some(EvalStackResolveResult { key, value, args }) = self.stack.resolve(curr) {
            match value {
                EvalDefValue::Base => return match args.len() {
                    0 => Some(EvalDefValue::Ref(curr.clone())),
                    _ => {
                        let key_cloned = key.clone();
                        let mut eval_args = HashMap::new();
                        for (param, arg) in args.into_iter() {
                            eval_args.insert(
                                param,
                                EvalIdentsKind::Inner({
                                    let arg_idents = match arg {
                                        EvalIdentsKind::Inner(inner) => inner,
                                        _ => vec![arg],
                                    };

                                    match self.eval_exprs(&arg_idents, false) {
                                        Some(EvalDefValue::Ref(idents)) => idents,
                                        Some(EvalDefValue::Expanded(idents)) => idents,
                                        _ => panic!("argument not found: {:?}", arg_idents),
                                    }
                                }),
                            );
                        }
                        let expanded = key_cloned.assign_params(&eval_args);
                        Some(EvalDefValue::Expanded(expanded))
                    }
                },
                EvalDefValue::Ref(next) | EvalDefValue::Expanded(next) => {
                    if debug {
                        println!("{}", next.simple_display());
                    }
                    curr = next;
                }
                EvalDefValue::Node { body, exprs } => {
                    let body = *body;
                    let exprs = *exprs;

                    let iter = body.iter();
                    self.stack.push_scope(iter.clone());

                    for (param, arg) in args {
                        self.stack.push_def(vec![EvalIdentsKind::Expr(param)], EvalDefValue::Ref(vec![arg]));
                    }

                    self.for_each(|_| ());

                    let exprs_idents = self.eval_idents(exprs, EvalIdentsIdentOption::AlwaysExpr);
                    let def_value = self
                        .eval_exprs(&exprs_idents, debug)
                        .expect(format!("identifiers not found: {exprs_idents:?}").as_str());

                    self.stack.pop_scope();

                    if debug {
                        match &def_value {
                            EvalDefValue::Ref(idents) | EvalDefValue::Expanded(idents) => println!("{}", idents.simple_display()),
                            _ => panic!("unexpected definition: {:?}", def_value)
                        }
                    }

                    return Some(def_value);
                }
            }
        }

        None
    }

    /// Evaluate the next node.
    pub fn step(&mut self, debug_options: EvalDebugOption) -> Option<()> {
        let node = self.stack.next()?;

        if debug_options.contains(EvalDebugOption::STACK) {
            println!("----------stack----------\n{:#?}", self.stack)
        };

        if debug_options.contains(EvalDebugOption::CALL) {
            println!("----------call-----------\n{}", node.simple_display());
        };

        match node {
            SemNode {
                value: SemNodeKind::Def { idents, body, exprs },
                ..
            } => {
                if idents.is_empty() {
                    return Some(());
                }

                let dbg = matches!(
                    idents.last().unwrap(),
                    SemNodeExpr { value: SemNodeExprKind::Ident(y), .. } if y == "dbg!",
                );

                if !body.is_empty() {
                    if exprs.is_empty() {
                        panic!("definition with a body must have expressions");
                    }

                    let def_idents = self.eval_idents(idents, EvalIdentsIdentOption::ResolveWithStack);
                    self.stack.push_def(def_idents, EvalDefValue::Node { body, exprs });
                } else {
                    let exprs_idents = self.eval_idents(exprs, EvalIdentsIdentOption::AlwaysExpr);
                    let def_value = self
                        .eval_exprs(&exprs_idents, dbg)
                        .expect(format!("identifiers not found: {exprs_idents:?}").as_str());

                    if !dbg {
                        let ident_len = idents.len();
                        let def_idents = self.eval_idents(
                            idents,
                            match def_value {
                                EvalDefValue::Base => match ident_len {
                                    1 => EvalIdentsIdentOption::AlwaysExpr,
                                    _ => EvalIdentsIdentOption::ResolveWithStack,
                                },
                                _ => EvalIdentsIdentOption::ResolveWithStack,
                            },
                        );
                        self.stack.push_def(def_idents, def_value);
                    }
                }
            }
            SemNode {
                value: SemNodeKind::Error { msg, .. },
                ..
            } => panic!("error: {}", msg),
        }
        Some(())
    }
}

impl<'a> Iterator for Evaluator<'a> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        self.step(self.debug_options)
    }
}

/// Evaluation identifiers to debug output trait.
impl SimpleDisplay for EvalIdents {
    fn simple_display(&self) -> String {
        self.iter()
            .map(|ident|
            match ident {
                EvalIdentsKind::Expr(ident) => ident.clone(),
                EvalIdentsKind::Inner(inner) => {
                    format!("({})", inner.simple_display())
                },
                _ => panic!("cannot print parameter: {:?}", ident),
            }
            )
            .collect::<Vec<String>>()
            .join(" ")
    }
}
