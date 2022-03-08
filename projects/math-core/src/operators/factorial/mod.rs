use bigdecimal_rs::ToPrimitive;
use num::{BigUint, One};

use crate::{ASTKind, ASTNode, Primitive, Result, Span, Symbolic};

mod convert;
mod traits;

#[derive(Debug)]
pub struct Factorial {}

impl Symbolic for Factorial {
    fn name(&self) -> &'static str {
        "Factorial"
    }

    fn apply(&self, span: Span, args: &[ASTNode]) -> Result<ASTNode> {
        let first = match args {
            [first] => first,
            _ => panic!("{:?}: Factorial takes at exactly one argument", span),
        };
        let out = match &first.kind {
            ASTKind::Atomic { atom } => atom.factorial(span)?,
            ASTKind::List { items } => {
                todo!()
            }
            ASTKind::Function { head, rest } => {
                todo!()
            }
        };
        Ok(out)
    }
}

pub trait FactorialFast {
    fn factorial(&self, span: Span) -> Result<ASTNode>;
}

impl FactorialFast for Primitive {
    fn factorial(&self, span: Span) -> Result<ASTNode> {
        let out = match self {
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
                // TODO: https://www.zhihu.com/question/489536779/answer/2342104485
                let n = i.to_usize().unwrap();
                let out = (1..=n).fold(BigUint::one(), |a, b| a * b);
                ASTNode::primitive(out, span)
            }
            Primitive::Decimal(_) => {
                todo!()
            }
        };
        Ok(out)
    }
}
