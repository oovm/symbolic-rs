use std::fmt::Debug;

use bigdecimal_rs::BigDecimal;
use num::{BigInt, BigUint};

use crate::{ASTKind, ASTNode, Result, Span, Symbolic};
mod convert;
mod traits;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Primitive {
    Boolean(bool),
    Character(char),
    String(String),
    Integer(BigInt),
    Decimal(BigDecimal),
}

impl ASTNode {
    pub fn primitive<T>(atom: T, span: Span) -> Self
    where
        T: Into<Primitive>,
    {
        ASTNode { kind: ASTKind::Atomic { atom: atom.into() }, span }
    }
}
