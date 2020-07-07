use crate::{
    traits::{Add, Connect, Div, Factorial, Mul, Pow, Sub, Surd},
    AST,
};

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

impl<T> Mul<AST<T>> for AST<T> {
    type Output = AST<T>;

    fn mul(self, rhs: AST<T>) -> Self::Output {
        AST::Times(box self, box rhs)
    }
}

impl<T> Div<AST<T>> for AST<T> {
    type Output = AST<T>;

    fn div(self, rhs: AST<T>) -> Self::Output {
        AST::Divide(box self, box rhs)
    }
}

impl<T> Pow<AST<T>> for AST<T> {
    type Output = AST<T>;

    fn pow(self, rhs: AST<T>) -> Self::Output {
        AST::Power(box self, box rhs)
    }
}

impl<T> AST<T>
where
    T: Clone
    + Factorial
    + Add<Output = T>
    + Sub<Output = T>
    + Mul<Output = T>
    + Div<Output = T>
    + Pow<T, Output = T>
    + Surd<Output = T>
    + Connect<Output = T>,
{
    pub fn eval(self) -> T {
        match self {
            AST::Number(n) => n,
            AST::Factorial(n) => n.eval().factorial(),
            //
            AST::Plus(a, b) => a.eval() + b.eval(),
            AST::Minus(a, b) => a.eval() - b.eval(),
            AST::Times(a, b) => a.eval() * b.eval(),
            AST::Divide(a, b) => a.eval() / b.eval(),
            AST::Power(a, b) => a.eval().pow(b.eval()),
            AST::Surd(a, b) => a.eval().surd(b.eval()),
            //
            AST::Connect(a, b) => a.eval().connect(b.eval()),
        }
    }
    pub fn unwrap(self) -> T {
        match self {
            AST::Number(n) => n,
            _ => unreachable!(),
        }
    }
}
