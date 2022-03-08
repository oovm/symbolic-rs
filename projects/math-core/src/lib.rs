use core::any::Any;
use std::{
    collections::BTreeMap,
    fmt::{Debug, Formatter},
};

use crate::primitive::Primitive;

pub(crate) mod factorial;
pub(crate) mod primitive;

pub trait Symbolic: Debug {
    fn name(&self) -> &'static str;
    fn apply(&self, span: Span, args: &[ASTNode]) -> ASTNode;
    fn as_any(&self) -> &dyn Any
    where
        Self: Sized + 'static,
    {
        self
    }
}

#[derive(Debug)]
pub enum ASTKind {
    Atomic { atom: Primitive },
    Function { head: Box<dyn Symbolic>, rest: Vec<ASTNode> },
}

#[derive(Debug, Copy, Clone)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Default for Span {
    fn default() -> Self {
        Span { start: 0, end: 0 }
    }
}

#[derive(Debug)]
pub struct ASTNode {
    kind: ASTKind,
    span: Span,
}

impl Debug for FactorInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FactorInteger({})", self.fast.len())
    }
}

pub struct FactorInteger {
    fast: BTreeMap<&'static str, Box<dyn Fn(Span, &[ASTNode]) -> ASTNode>>,
}

impl FactorInteger {
    pub fn new_fast(&mut self, name: &'static str, func: impl Fn(Span, &[ASTNode]) -> ASTNode + 'static) {
        self.fast.insert(name, Box::new(func));
    }
    pub fn try_fast(&self, name: &str, span: Span, args: &[ASTNode]) -> Option<ASTNode> {
        self.fast.get(name).map(|f| f(span, args))
    }
    pub fn builtin() -> Self {
        let mut base = FactorInteger { fast: BTreeMap::new() };
        base.new_fast("Factorial", |span, args| ASTNode { kind: ASTKind::Atomic { atom: Primitive::from(2) }, span });
        base
    }
}

impl ASTNode {
    pub fn apply(&self) -> ASTNode {
        self.kind.eval(self.span)
    }
}

impl ASTKind {
    pub fn eval(&self, span: Span) -> ASTNode {
        match self {
            Self::Function { head, rest } => head.apply(span, rest),
            Self::Atomic { atom } => atom.apply(span, &[]),
        }
    }
}

impl Symbolic for FactorInteger {
    fn name(&self) -> &'static str {
        "FactorInteger"
    }

    fn apply(&self, span: Span, args: &[ASTNode]) -> ASTNode {
        match &args[0].kind {
            ASTKind::Function { head, rest } => {
                self.try_fast(head.name(), span, rest).unwrap_or_else(|| head.apply(span, rest))
            }
            ASTKind::Atomic { atom } => self.try_fast(atom.name(), span, &[]).unwrap_or_else(|| atom.apply(span, &[])),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[test]
fn test() {
    let bigint = ASTNode { kind: ASTKind::Atomic { atom: Primitive::from(1) }, span: Span::default() };

    let factorial = Factorial {};
    let factor_integer = FactorInteger::builtin();
    let factorial_node =
        ASTNode { kind: ASTKind::Function { head: Box::new(factorial), rest: vec![bigint] }, span: Span::default() };
    let factor_integer_node = ASTNode {
        kind: ASTKind::Function { head: Box::new(factor_integer), rest: vec![factorial_node] },
        span: Span::default(),
    };
    println!("{:?}", factor_integer_node.apply())
}
