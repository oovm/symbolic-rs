pub trait Constant {
    fn eval(&self) -> ASTNode;
}

pub trait Function {
    fn eval(&self, args: &[ASTNode]) -> ASTNode;
}


pub enum ASTKind {
    Constant {
        inner: Box<dyn Constant>,
    },
    Function {
        head: Box<dyn Function>,
        rest: Vec<ASTNode>,
    },
}

pub struct Span {
    pub start: u32,
    pub end: u32,
}

pub struct ASTNode {
    kind: ASTKind,
    span: Span,
}

pub struct FactorInteger {}

pub struct Factorial {}

impl ASTKind {
    pub fn eval(&self) -> ASTKind {
        match self {
            Self::Constant { .. } => {}
            Self::Function { .. } => {}
        }
    }
}

impl Function for Factorial {
    fn eval(&self, args: &[ASTNode]) -> ASTNode {
        todo!()
    }
}


impl Function for FactorInteger {
    fn eval(&self, args: &[ASTNode]) -> ASTNode {
        todo!()
    }
}


