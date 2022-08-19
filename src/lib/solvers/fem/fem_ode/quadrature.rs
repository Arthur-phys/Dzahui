
/// # General Information
/// 
/// A quadrature represents a process to obtain per-interval areas that (naturally) summed can approximate an integral. The current input consits on a vector
/// of vectors of functions, each that represents a derivate of a given set of functions (commonly a basis), but probably will be changed in the future allowing
/// to only provide a vector of functions each wit a trait representing differentiability.
///  
trait Quadrature1D {
    
    /// # General Information
    /// 
    /// Integrates a given set of functions according to a solver's instructions.
    /// 
    /// # Parameters
    /// 
    /// * `functions` - a vector of vector of functions, better described (mathematically) as a vector of a function set and all the necessary derivatives of such
    /// a set.
    /// 
    fn integrate(functions: Vec<Vec<Box<dyn Fn(f32) -> f32>>>) -> f32;
}
trait NewtonCotes: Quadrature1D {}

struct GaussLegendreQuadatrure {}
struct SimpsonQuadrature {}
struct TrapezoidQuadrature {}

impl Quadrature1D for GaussLegendreQuadatrure {

    fn integrate(functions: Vec<Vec<Box<dyn Fn(f32) -> f32>>>) -> f32 {
        todo!()
    }
}

impl Quadrature1D for SimpsonQuadrature {

    fn integrate(functions: Vec<Vec<Box<dyn Fn(f32) -> f32>>>) -> f32 {
        todo!()
    }
}

impl Quadrature1D for TrapezoidQuadrature {

    fn integrate(functions: Vec<Vec<Box<dyn Fn(f32) -> f32>>>) -> f32 {
        todo!()
    }
}

impl NewtonCotes for SimpsonQuadrature {}
impl NewtonCotes for TrapezoidQuadrature {}