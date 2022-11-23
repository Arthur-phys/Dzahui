// Module definition
pub mod euler;
pub mod fem;
pub mod matrix_solver;
pub mod quadrature;
pub mod solver_trait;

// Re-exports
pub use fem::Solver;
pub use fem::*;
