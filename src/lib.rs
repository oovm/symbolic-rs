#![feature(box_syntax)]

mod traits;
mod utils;

#[derive(Clone, Debug)]
pub enum AST<T> {
    Number(T),
    Plus(Box<AST<T>>, Box<AST<T>>),
    Minus(Box<AST<T>>, Box<AST<T>>),
    Times(Box<AST<T>>, Box<AST<T>>),
    Divide(Box<AST<T>>, Box<AST<T>>),
    Power(Box<AST<T>>, Box<AST<T>>),
    Surd(Box<AST<T>>, Box<AST<T>>),
}
