use bigdecimal_rs::ToPrimitive;
use num::{BigUint, One};

use crate::{ASTKind, ASTNode, Primitive, Span, Symbolic};

mod convert;
mod traits;

#[derive(Debug)]
pub struct Factorial {}

impl Symbolic for Factorial {
    fn name(&self) -> &'static str {
        "Factorial"
    }

    fn apply(&self, span: Span, args: &[ASTNode]) -> ASTNode {
        let first = match args {
            [first] => first,
            _ => panic!("{:?}: Factorial takes at exactly one argument", span),
        };
        match &first.kind {
            ASTKind::Atomic { atom } => {
                todo!()
            }
            ASTKind::Function { head, rest } => {
                todo!()
            }
        }
    }
}

pub trait FactorialFast {
    fn factorial(&self, span: Span) -> ASTNode;
}

impl FactorialFast for Primitive {
    fn factorial(&self, span: Span) -> ASTNode {
        match self {
            Primitive::Boolean(_) => {
                todo!()
            }
            Primitive::Character(_) => {
                todo!()
            }
            Primitive::String(_) => {
                todo!()
            }
            Primitive::Integer(i) => {
                let n = i.to_usize().unwrap();
                let out = (1..=n).fold(BigUint::one(), |a, b| a * b);
                ASTNode::primitive(out, span)
            }
            Primitive::Decimal(_) => {
                todo!()
            }
        }
    }
}
