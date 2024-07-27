use std::collections::HashMap;

/// Evaluation identifier kind.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum EvalIdentsKind {
    Expr(String),
    Param(String),
    Inner(EvalIdents),
}

/// Evaluation identifiers.
pub type EvalIdents = Vec<EvalIdentsKind>;

/// Evaluation identifiers extensions trait.
pub trait EvalIdentsExtensions {
    /// Check if the argument matches this identifier.
    /// The argument must not contain any parameters.
    fn matches(&self, idents: &EvalIdents) -> Option<HashMap<String, EvalIdentsKind>>;

    /// Assign arguments to parameters.
    fn assign_params(self, args: &HashMap<String, EvalIdentsKind>) -> EvalIdents;
}

impl EvalIdentsExtensions for EvalIdents {
    fn matches(&self, idents: &EvalIdents) -> Option<HashMap<String, EvalIdentsKind>> {
        if idents.len() == 0 || self.len() != idents.len() {
            return None;
        }

        let mut params = HashMap::new();

        for (a, b) in self.iter().zip(idents.iter()) {
            match (a, b) {
                (_, EvalIdentsKind::Param(_)) => {
                    panic!("idents contain parameters");
                }
                (EvalIdentsKind::Expr(a), EvalIdentsKind::Expr(b)) => {
                    if a != b {
                        return None;
                    }
                }
                (EvalIdentsKind::Inner(a), EvalIdentsKind::Inner(b)) => match a.matches(b) {
                    Some(child_params) => params.extend(child_params),
                    _ => return None,
                },
                (EvalIdentsKind::Param(a), b) => {
                    if let Some(old) = params.insert(a.clone(), b.clone()) {
                        panic!("parameter already exist: {:?}", old);
                    }
                }
                _ => return None,
            }
        }

        Some(params)
    }

    fn assign_params(self, args: &HashMap<String, EvalIdentsKind>) -> EvalIdents {
        self.into_iter()
            .map(|ident| match ident {
                EvalIdentsKind::Param(param) => match args.get(&param) {
                    Some(ident) => ident.clone(),
                    None => panic!("argument not found for parameter: {:?}", param),
                },
                EvalIdentsKind::Inner(inner) => EvalIdentsKind::Inner(inner.assign_params(args)),
                _ => ident.clone(),
            })
            .collect()
    }
}
