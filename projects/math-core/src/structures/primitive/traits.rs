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

    fn forward(&self, span: Span, args: &[ASTNode]) -> Result<ASTNode> {
        debug_assert!(args.is_empty());
        Ok(ASTNode::primitive(self.clone(), span))
    }
}
