// Module definition
mod error;
pub(crate) mod mesh;
pub mod simulation;
pub mod solvers;
pub mod logger;

// Re-exports
pub use self::error::Error;
pub use self::simulation::dzahui_window::{DzahuiWindow, DzahuiWindowBuilder};
pub use self::solvers::euler::EulerSolver;
