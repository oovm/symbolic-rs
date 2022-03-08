use std::fmt::{Debug, Formatter};

use bigdecimal_rs::BigDecimal;
use num::{BigInt, BigRational, BigUint};

use crate::{ASTKind, ASTNode, Span, Symbolic};

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

impl Symbolic for Primitive {
    fn name(&self) -> &'static str {
        match self {
            Primitive::Boolean(_) => "Boolean",
            Primitive::String(_) => "String",
            Primitive::Integer(_) => "Integer",
            Primitive::Decimal(_) => "Decimal",
            Primitive::Character(_) => "Character",
        }
    }

    fn apply(&self, span: Span, args: &[ASTNode]) -> ASTNode {
        debug_assert!(args.is_empty());
        ASTNode { kind: ASTKind::Atomic { atom: self.clone() }, span }
    }
}
