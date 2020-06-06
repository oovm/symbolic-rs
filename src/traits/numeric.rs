use std::ops::{Add, Sub};
use crate::AST;

impl<T> Add<AST<T>> for AST<T> {
    type Output = AST<T>;

    fn add(self, rhs: AST<T>) -> Self::Output {
        AST::Plus(box self, box rhs)
    }
}

impl<T> Sub<AST<T>> for AST<T> {
    type Output = AST<T>;

    fn sub(self, rhs: AST<T>) -> Self::Output {
        AST::Minus(box self, box rhs)
    }
}


#[test]
fn test() {
    println!("{:?}", AST::Number(1) + AST::Number(2));
    println!("{:?}", AST::Number(1) - AST::Number(2))
}
