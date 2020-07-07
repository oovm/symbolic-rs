mod display;
mod numeric;

pub use factorial::Factorial;
pub use num::traits::Pow;
pub use std::ops::{Add, Div, Mul, Sub};

pub trait Surd<Rhs = Self> {
    type Output;

    fn surd(self, rhs: Rhs) -> Self::Output;
}

pub trait Connect<Rhs = Self> {
    type Output;

    fn connect(self, rhs: Rhs) -> Self::Output;
}
