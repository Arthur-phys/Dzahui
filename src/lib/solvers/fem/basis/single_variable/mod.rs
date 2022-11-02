// Module definition
pub mod linear_basis;
pub mod piecewise_polynomials_1d;
pub mod polynomials_1d;
pub mod quadratic_basis;

/// # General Information
///
/// Every struct defined to behave like a function must have a way to be evaluated. In one dimension, all it takes is to obtain a value x
/// and return another value y.
///
/// # Functions
///
/// * `evaluate(...)` - Evaluation of a 1D function.
///
pub trait Function1D {
    /// Evaluation of a 1D function.
    fn evaluate(&self, x: f64) -> f64;
}

/// # General Information
///
/// Every differentiable function-like struct must implement this trait. It is important to know which kind of function will result from differentiating.
/// Such a function needs to become a struct that represents a family of it's kind and also implements the trait `Function1D`.
///
/// # Functions
///
/// * `differentiate(...)` - Results in a function product of differentiation.
///
pub trait Differentiable1D<T>: Function1D
where
    T: Function1D,
{
    /// Results in a function product of differentiation.
    fn differentiate(&self) -> T;
}

/// # General Information
///
/// The composition, given `f`, the function on which this trait is implemented, and `g`, the function inside the trait's only function, is done like:
/// ```f(g(x))```
/// All composable functions need to implement this trait. When a functions needs to be composed with another, the latter one needs to be specified a priori,
/// and the result of the composition needs to be known. This means that the trait Composable<latter, result> needs to be implemented.
///
/// # Functions
///
/// * `compose(...)` - Returns a function from the composition of the two functions involved. Consumes functions.
pub trait Composable1D<T, U>
where
    T: Function1D,
    U: Function1D,
{
    /// Returns a function from the composition of the two functions involved. Consumes functions
    fn compose(self, other: T) -> U;
}
