
trait Quadrature1D {
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