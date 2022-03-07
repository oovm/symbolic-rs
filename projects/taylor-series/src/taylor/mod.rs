

pub trait Taylor {
    fn taylor_series(self, n: usize) -> f64;
}

pub struct TaylorSeries<'a> {

}


/// {\displaystyle e^{x}=\sum _{n=0}^{\infty }{\frac {x^{n}}{n!}}=1+x+{\frac {x^{2}}{2!}}+{\frac {x^{3}}{3!}}+\cdots }.
pub struct Sin {
    pub x: f64,
}

