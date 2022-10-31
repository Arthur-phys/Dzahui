pub mod linear_basis;
pub mod piecewise_polynomials_1d;
pub mod polynomials_1d;
pub mod quadratic_basis;

pub trait Function1D {
    fn evaluate(&self, x: f64) -> f64;
}

pub trait Differentiable1D<T>
where
    T: Function1D,
{
    fn differentiate(&self) -> T;
}

pub trait Composable1D<T, U>
where
    T: Function1D,
    U: Function1D,
{
    fn compose(self, other: T) -> U;
}
