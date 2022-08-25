pub mod gauss_legendre;

/// # General Information
/// 
/// A quadrature represents a process to obtain per-interval areas that (naturally) summed can approximate an integral. The current input consits on a vector
/// of vectors of function, each that represents a derivate of a given set of function (commonly a basis), but probably will be changed in the future allowing
/// to only provide a vector of function each wit a trait representing differentiability.
///  
trait Quadrature1D {
    
    /// # General Information
    /// 
    /// Integrates a given set of function according to a solver's instructions.
    /// 
    /// # Parameters
    /// 
    /// * `function` - a vector of vector of function, better described (mathematically) as a vector of a function set and all the necessary derivatives of such
    /// a set.
    /// 
    fn integrate(function: Box<dyn Fn(f32) -> f32>) -> f32;
}
trait NewtonCotes: Quadrature1D {}
struct SimpsonQuadrature {}
struct TrapezoidQuadrature {}

impl Quadrature1D for SimpsonQuadrature {

    fn integrate(function: Box<dyn Fn(f32) -> f32>) -> f32 {
        todo!()
    }
}

impl Quadrature1D for TrapezoidQuadrature {

    fn integrate(function: Box<dyn Fn(f32) -> f32>) -> f32 {
        todo!()
    }
}

impl NewtonCotes for SimpsonQuadrature {}
impl NewtonCotes for TrapezoidQuadrature {}