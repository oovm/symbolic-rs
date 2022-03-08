use crate::{ASTKind, ASTNode, Span, Symbolic};
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
