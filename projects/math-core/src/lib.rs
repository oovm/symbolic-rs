use std::fmt::{Debug, Formatter};
use downcast_trait::{downcast_trait, downcast_trait_impl_convert_to, DowncastTrait};
use core::{any::{Any, TypeId}, mem};
use std::collections::BTreeMap;
use std::hash::Hash;
use num::BigInt;

pub trait Symbolic: Debug {
    fn name(&self) -> &'static str;
    fn eval(&self, span: Span, args: &[ASTNode]) -> ASTNode;
    fn as_any(&self) -> &dyn Any where Self: Sized + 'static {
        self
    }
}



#[derive(Debug)]
pub enum ASTKind {
    Atom {
        atom: Box<dyn Symbolic>,
    },
    Function {
        head: Box<dyn Symbolic>,
        rest: Vec<ASTNode>,
    },
}

#[derive(Debug, Copy, Clone)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Default for Span {
    fn default() -> Self {
        Span {
            start: 0,
            end: 0,
        }
    }
}

#[derive(Debug)]
pub struct ASTNode {
    kind: ASTKind,
    span: Span,
}

impl Debug for FactorInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FactorInteger({})", self.fast.len())
    }
}

pub struct FactorInteger {
    fast: BTreeMap<&'static str, Box<dyn Fn(Span, &[ASTNode]) -> ASTNode>>,
}

impl FactorInteger {
    pub fn new_fast(&mut self, name: &'static str, func: impl Fn(Span, &[ASTNode]) -> ASTNode + 'static) {
        self.fast.insert(name, Box::new(func));
    }
    pub fn try_fast(&self, name: &str, span: Span, args: &[ASTNode]) -> Option<ASTNode> {
        self.fast.get(name).map(|f| f(span, args))
    }
    pub fn builtin() -> Self {
        let mut base = FactorInteger {
            fast: BTreeMap::new(),
        };
        base.new_fast("Factorial", |span, args| {
            ASTNode {
                kind: ASTKind::Atom {
                    atom: Box::new(BigInt::from(2)),
                },
                span: Default::default()
            }
        });
        base

    }
}

#[derive(Debug)]
pub struct Factorial {}

impl ASTNode {
    pub fn eval(&self) -> ASTNode {
        self.kind.eval(self.span)
    }
}

impl ASTKind {
    pub fn eval(&self, span: Span) -> ASTNode {
        match self {
            Self::Function { head, rest } => {
                head.eval(span, rest)
            }
            Self::Atom { atom } => {
                atom.eval(span, &[])
            }
        }
    }
}

impl Symbolic for BigInt {
    fn name(&self) -> &'static str {
        "Integer"
    }

    fn eval(&self, span: Span, args: &[ASTNode]) -> ASTNode {
        ASTNode {
            kind: ASTKind::Atom {
                atom: Box::new(self.clone()),
            },
            span,
        }
    }
}

impl Symbolic for Factorial {
    fn name(&self) -> &'static str {
        "Factorial"
    }

    fn eval(&self, span: Span, args: &[ASTNode]) -> ASTNode {
        args[0].eval()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}


impl Symbolic for FactorInteger {
    fn name(&self) -> &'static str {
        "FactorInteger"
    }

    fn eval(&self, span: Span, args: &[ASTNode]) -> ASTNode {
        match &args[0].kind {
            ASTKind::Function { head, rest } => {
                self.try_fast(head.name(), span, rest).unwrap_or_else(|| {
                    head.eval(span, rest)
                })
            }
            ASTKind::Atom { atom } => {
                self.try_fast(atom.name(), span, &[]).unwrap_or_else(|| {
                    atom.eval(span, &[])
                })
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[test]
fn test() {
    let bigint = BigInt::from(1);
    let bigint = ASTNode {
        kind: ASTKind::Atom {
            atom: Box::new(bigint)
        },
        span: Span::default(),
    };

    let factorial = Factorial {};
    let factor_integer = FactorInteger::builtin();
    let factorial_node = ASTNode {
        kind: ASTKind::Function {
            head: Box::new(factorial),
            rest: vec![bigint],
        },
        span: Span::default(),
    };
    let factor_integer_node = ASTNode {
        kind: ASTKind::Function {
            head: Box::new(factor_integer),
            rest: vec![factorial_node],
        },
        span: Span::default(),
    };
    println!("{:?}", factor_integer_node.eval())
}

