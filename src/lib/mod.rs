// Module definition
pub mod simulation;
pub mod solvers;
pub(crate) mod mesh;
mod error;

// Re-exports
pub use self::error::Error;
pub use self::simulation::dzahui_window::{DzahuiWindow, DzahuiWindowBuilder};
pub use self::solvers::{euler::EulerSolver};
