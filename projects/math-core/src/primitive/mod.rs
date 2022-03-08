use std::fmt::Debug;

use bigdecimal_rs::BigDecimal;
use num::{BigInt, BigUint};

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
