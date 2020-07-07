#![feature(box_syntax)]

pub mod traits;
mod utils;

#[derive(Clone, Debug)]
pub enum AST<T> {
    Number(T),
    Factorial(Box<AST<T>>),
    Plus(Box<AST<T>>, Box<AST<T>>),
    Minus(Box<AST<T>>, Box<AST<T>>),
    Times(Box<AST<T>>, Box<AST<T>>),
    Divide(Box<AST<T>>, Box<AST<T>>),
    Power(Box<AST<T>>, Box<AST<T>>),
    Surd(Box<AST<T>>, Box<AST<T>>),
    Connect(Box<AST<T>>, Box<AST<T>>),
}
