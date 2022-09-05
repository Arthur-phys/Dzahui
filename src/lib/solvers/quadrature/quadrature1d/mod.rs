pub mod gauss_legendre;

pub use gauss_legendre::GaussLegendreQuadrature;

trait SimpsonQuadrature {}
trait TrapezoidQuadrature {}