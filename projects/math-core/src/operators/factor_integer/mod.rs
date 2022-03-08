use crate::{ASTKind, ASTNode, Primitive, Result, SMError, Span, Symbolic};
use std::{
    any::Any,
    collections::{BTreeMap, VecDeque},
    fmt::{Debug, Formatter},
};

pub struct FactorInteger {
    fast: BTreeMap<&'static str, Box<dyn Fn(Span, &[ASTNode]) -> ASTNode>>,
}

impl FactorInteger {
    pub fn new_fast(&mut self, name: &'static str, func: impl Fn(Span, &[ASTNode]) -> ASTNode + 'static) {
        self.fast.insert(name, Box::new(func));
    }
    pub fn try_fast(&self, name: &str, span: Span, args: &[ASTNode]) -> Result<ASTNode> {
        match self.fast.get(name) {
            None => Err(SMError::new(span, format!("Unknown function: {}", name))),
            Some(f) => Ok(f(span, args)),
        }
    }
    pub fn builtin() -> Self {
        let mut base = FactorInteger { fast: BTreeMap::new() };
        base.new_fast("Factorial", |span, args| ASTNode { kind: ASTKind::Atomic { atom: Primitive::from(2) }, span });
        base
    }
}

impl Debug for FactorInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FactorInteger({})", self.fast.len())
    }
}

impl Symbolic for FactorInteger {
    fn name(&self) -> &'static str {
        "FactorInteger"
    }

    fn apply(&self, span: Span, args: &[ASTNode]) -> Result<ASTNode> {
        let out = match &args[0].kind {
            ASTKind::Atomic { atom } => self.try_fast(atom.name(), span, &[]).or_else(|_| atom.apply(span, &[]))?,
            ASTKind::List { items } => {
                let mut out = VecDeque::with_capacity(items.len());
                for item in items {
                    out.push_back(item.apply()?);
                }
                ASTNode { kind: ASTKind::List { items: out }, span }
            }
            ASTKind::Function { head, rest } => self.try_fast(head.name(), span, rest).or_else(|_| head.apply(span, rest))?,
        };
        Ok(out)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
