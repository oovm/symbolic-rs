use super::*;

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
