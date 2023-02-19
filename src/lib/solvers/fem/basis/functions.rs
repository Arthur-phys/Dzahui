use crate::Error;

/// # General Information
///
/// Every struct defined to behave like a function must have a way to be evaluated. In one dimention, all it takes is to obtain a value x
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
/// Every struct defined to behave like a function must have a way to be evaluated. In two dimentions, all it takes is to obtain a value (x,y)
/// and return another value (z,w).
///
/// # Functions
///
/// * `evaluate(...)` - Evaluation of a 2D function.
///
pub trait Function2D {
    /// Evaluation of a 2D function.
    fn evaluate(&self, x: f64, y: f64) -> f64;
}

/// # General Information
///
/// Every differentiable 1D function-like struct must implement this trait. It is important to know which kind of function will result from differentiating.
/// Such a function needs to become a struct that represents a family of it's kind and also implements the trait `Function1D`.
///
/// # Functions
///
/// * `differentiate(...)` - Results in a function product of differentiation.
///
pub trait Differentiable1D<T>: Function1D
where
    T: Function1D
{
    /// Results in a function product of differentiation.
    fn differentiate(&self) -> Result<T,Error>;
}

/// # General Information
///
/// Every differentiable 2D function-like struct must implement this trait. It is important to know which kind of function will result from differentiating.
/// Such a function needs to become a struct that represents a family of it's kind and also implements the trait `Function2D`.
/// A function completely differentiable in 2 dimentions is differentiable on every entry. The converse is not always true.
///
/// # Functions
///
/// * `differentiate_x(...)` - Differentiate with respect to x
/// * `differentiate_y(...)` - Differentiate with respect to y
///
pub trait Differentiable2D<T,V>: Function2D
where 
    T: Function2D,
    V: Function2D
{
    fn differentiate_x(&self) -> Result<T,Error>;
    fn differentiate_y(&self) -> Result<T,Error>;
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
    fn compose(self, other: T) -> Result<U,Error>;
}

/// # General Information
///
/// The composition, given `f`, the function on which this trait is implemented, and `g`, the function inside the trait's only function, is done like:
/// ```f(g(x,y))```
/// All composable functions need to implement this trait. When a functions needs to be composed with another, the latter one needs to be specified a priori,
/// and the result of the composition needs to be known. This means that the trait Composable<latter, result> needs to be implemented.
///
/// # Functions
///
/// * `compose(...)` - Returns a function from the composition of the two functions involved. Consumes functions.
pub trait Composable2D<T,V>
where
    T: Function2D,
    V: Function2D
{
    fn compose(self,other: T) -> Result<V,Error>;
}
