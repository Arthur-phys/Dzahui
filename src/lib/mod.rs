pub mod simulation;
pub mod solvers;
pub(crate) mod mesh;
mod error;

// Reimports
pub use self::simulation::dzahui_window::{DzahuiWindow, DzahuiWindowBuilder};
pub use self::solvers::euler::EulerSolver;
pub use self::error::Error;