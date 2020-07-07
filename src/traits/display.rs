use crate::AST;
use std::fmt::{self, Display, Formatter};

fn o2<T: Clone + Display>(ast: &Box<AST<T>>) -> String {
    match **ast {
        AST::Plus(..) | AST::Minus(..) => format!("({})", ast),
        _ => format!("{}", ast),
    }
}

fn o3<T: Clone + Display>(ast: &Box<AST<T>>) -> String {
    match **ast {
        AST::Plus(..) | AST::Minus(..) | AST::Times(..) | AST::Divide(..) => format!("({})", ast),
        _ => format!("{}", ast),
    }
}

impl<T: Clone + Display> Display for AST<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            AST::Number(n) => write!(f, "{}", n),
            AST::Factorial(n) => write!(f, "{}!", n),
            //
            AST::Plus(l, r) => write!(f, "{} + {}", l, r),
            AST::Minus(l, r) => write!(f, "{} - {}", l, r),
            AST::Times(l, r) => write!(f, "{} × {}", o2(l), o2(r)),
            AST::Divide(l, r) => write!(f, "{} ÷ {}", o2(l), o2(r)),
            AST::Power(l, r) => write!(f, "{}^{}", o3(l), o3(r)),
            AST::Surd(l, r) => write!(f, "{}√{}", o3(l), o3(r)),
            //
            AST::Connect(l, r) => write!(f, "{}{}", l, r),
        }
    }
}
