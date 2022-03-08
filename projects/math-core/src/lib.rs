use core::any::Any;
use std::{collections::VecDeque, fmt::Debug};

pub use errors::{Result, SMError};

use self::{operators::*, structures::*};

mod errors;
mod operators;
mod structures;
mod traits;

pub trait Symbolic: Debug {
    fn name(&self) -> &'static str;
    fn apply(&self, span: Span, args: &[ASTNode]) -> Result<ASTNode>;
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
    List { items: VecDeque<ASTNode> },
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

impl ASTNode {
    pub fn apply(&self) -> Result<ASTNode> {
        self.kind.eval(self.span)
    }
}

impl ASTKind {
    pub fn eval(&self, span: Span) -> Result<ASTNode> {
        let out = match self {
            Self::Atomic { atom } => atom.apply(span, &[])?,
            Self::List { items } => {
                todo!()
            }
            Self::Function { head, rest } => head.apply(span, rest)?,
        };
        Ok(out)
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
