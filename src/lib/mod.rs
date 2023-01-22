// Module definition
mod error;
pub(crate) mod mesh;
pub mod simulation;
pub mod solvers;
pub mod logger;
pub(crate) mod writer;

// Re-exports
pub use self::error::Error;
pub use self::simulation::dzahui_window::{DzahuiWindow, DzahuiWindowBuilder};
pub use self::solvers::euler::EulerSolver;
pub use self::solvers::diffusion_solver::DiffussionParams;
pub use self::solvers::navier_stokes_solver::NavierStokesParams;
