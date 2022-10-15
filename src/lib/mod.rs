mod error;
pub(crate) mod mesh;
pub mod simulation;
pub mod solvers;

// Reimports
pub use self::error::Error;
pub use self::simulation::dzahui_window::{DzahuiWindow, DzahuiWindowBuilder};
pub use self::solvers::euler::EulerSolver;
