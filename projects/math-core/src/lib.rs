use std::any::Any;
use std::fmt::{Debug, Formatter};

pub trait Symbol: DowncastTrait {
    fn eval(&self, span: Span, args: &[ASTNode]) -> ASTNode;
    fn as_any(&self) -> &dyn Any;
}

impl Debug for Box<dyn Symbol> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Function")
    }
}

#[derive(Debug)]
pub enum ASTKind {
    Function {
        head: Box<dyn Symbol>,
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

pub trait FactorIntegerFast {
    fn factor_integer_fast(&self, span: Span, args: &[ASTNode]) -> ASTNode;
}

#[derive(Debug)]
pub struct FactorInteger {}

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
        }
    }
}

impl FactorIntegerFast for Factorial {
    fn factor_integer_fast(&self, span: Span, args: &[ASTNode]) -> ASTNode {
        todo!("should stop here")
    }
}

impl Symbol for Factorial {
    fn eval(&self, span: Span, args: &[ASTNode]) -> ASTNode {
        todo!("dont stop here")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}


impl Symbol for FactorInteger {
    fn eval(&self, span: Span, args: &[ASTNode]) -> ASTNode {
        match &args[0].kind {
            ASTKind::Function { head, rest } => {
                println!("{:?}", head.as_any().is::<&dyn FactorIntegerFast>());
                println!("{:?}", head.as_any().is::<Box<dyn FactorIntegerFast>>());
                println!("{:?}", head.as_any().is::<&Box<dyn FactorIntegerFast>>());

                match head.as_any().downcast_ref::<&dyn FactorIntegerFast>() {
                    Some(o) => { o.factor_integer_fast(span, rest) }
                    _ => { head.eval(span, rest) }
                }
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[test]
fn test() {
    let factorial = Factorial {};
    let factor_integer = FactorInteger {};
    let factorial_node = ASTNode {
        kind: ASTKind::Function {
            head: Box::new(factorial),
            rest: vec![],
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


#[macro_use] extern crate downcast_trait;
use downcast_trait::DowncastTrait;
use core::{any::{Any, TypeId}, mem};
trait Widget: DowncastTrait {}
trait Container: Widget {
    fn enumerate_widget_leaves_recursive(&self) -> Vec<&Box<dyn Widget>>;
}
struct Window {
    sub_widgets: Vec<Box<dyn Widget>>,
}
impl Widget for Window {}
impl Container for Window {
    fn enumerate_widget_leaves_recursive(&self) -> Vec<&Box<dyn Widget>> {
        let mut result = Vec::<&Box<dyn Widget>>::new();
        self.sub_widgets.iter().for_each(|sub_widget| {
            if let Some(sub_container) =
            downcast_trait!(dyn Container, sub_widget.as_ref().to_downcast_trait())
            {
                result.extend(sub_container.enumerate_widget_leaves_recursive());
            } else {
                result.push(sub_widget);
            }
        });
        result
    }
}
impl DowncastTrait for Window {
    downcast_trait_impl_convert_to!(dyn Container);
}