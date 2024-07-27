use std::collections::HashMap;

use crate::{EvalDefValue, EvalIdents, EvalIdentsExtensions, EvalIdentsKind};
use crate::evaluator::AdvanceSemNodeIterator;
use crate::parsers::SemNode;

/// Definition stack resolution result.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EvalStackResolveResult<'def, 'stack> {
    pub key: &'stack EvalIdents,
    pub value: &'stack EvalDefValue<'def>,
    pub args: HashMap<String, EvalIdentsKind>,
}

/// Definition stack item.
#[derive(Debug)]
pub struct EvalStackItem<'a> {
    pub scope: Vec<(EvalIdents, EvalDefValue<'a>)>,
    pub iter: Box<dyn AdvanceSemNodeIterator<'a> + 'a>,
}

/// Definition stack.
pub struct EvalStack<'a> {
    stack: Vec<EvalStackItem<'a>>,
}

impl<'a> EvalStack<'a> {
    /// Create a new definition stack.
    pub fn new<Iter>(iter: Iter) -> Self where Iter: AdvanceSemNodeIterator<'a> + 'a {
        Self {
            stack: vec![EvalStackItem {
                scope: vec![],
                iter: Box::new(iter),
            }],
        }
    }

    /// Push a new scope onto the stack.
    pub fn push_scope<Iter>(&mut self, iter: Iter) where Iter: AdvanceSemNodeIterator<'a> + 'a {
        self.stack.push(EvalStackItem {
            scope: vec![],
            iter: Box::new(iter),
        });
    }

    /// Pop a new scope from the stack.
    pub fn pop_scope(&mut self) -> Option<EvalStackItem<'a>> {
        self.stack.pop()
    }

    /// Push a new definition onto the stack.
    pub fn push_def(&mut self, key: EvalIdents, value: EvalDefValue<'a>) {
        if key.iter().all(|x| matches!(x, EvalIdentsKind::Param(_))) {
            panic!("a definition must have at least one non-parameter identifier")
        }

        self.stack
            .last_mut()
            .expect("scope is in stack")
            .scope
            .push((key, value));
    }

    /// Resolve an identifier.
    pub fn resolve<'stack>(&'stack self, ident: &EvalIdents) -> Option<EvalStackResolveResult<'a, 'stack>> {
        if ident.len() == 0 {
            return None;
        }

        for items in self.stack.iter().rev() {
            for (key, value) in items.scope.iter().rev() {
                if let Some(args) = key.matches(ident) {
                    return Some(EvalStackResolveResult { key, value, args });
                }
            }
        }

        None
    }
}

impl<'a> Iterator for EvalStack<'a> {
    type Item = &'a SemNode;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.last_mut().expect("scope in stack").iter.next()
    }
}

impl<'a> std::fmt::Debug for EvalStack<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        struct KeyValuePair<'a> {
            key: &'a EvalIdents,
            value: &'a EvalDefValue<'a>,
        }

        impl std::fmt::Debug for KeyValuePair<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(format_args!("{:?}: {:?}", self.key, self.value))
            }
        }

        f.debug_list()
            .entries(self.stack.iter().map(|x| {
                x.scope.iter()
                    .map(|(key, value)| KeyValuePair { key, value })
                    .collect::<Vec<_>>()
            }))
            .finish()
    }
}
