// Module declarations
pub mod basis;
pub mod utils;
pub mod diffusion_solver;
pub mod navier_stokes_solver;

// Internal dependencies + re-exports
pub use diffusion_solver::{DiffussionParamsTimeDependent, DiffussionSolverTimeDependent, DiffussionSolverTimeIndependent, DiffussionParamsTimeIndependent};
use super::solver_trait::DiffEquationSolver;

/// # General Information
///
/// An enum representing every equation implemented by this library.
/// Used as a way of representing the equation without having to create an instance of a solver of it. May be deprecated in the future in favor of a intercative
/// approach from window interface.
///
/// # Arms
///
/// * `DiffussionSolverTimeIndependent` - Diffusion equation solver representation.
/// * `DiffussionSolverTimeDependent` - Diffusion equation solver with time derivative representation.
/// * `NavierStokes1DSolver` - Navier Stokes in 1D solver representation.
/// * `NavierStokes2DSolver` - Navier Stokes in 2D solver representation.
/// * `None` - Visuallization of mesh in simulation. No equation attached.
///
#[derive(Debug)]
pub enum Solver {
    DiffussionSolverTimeIndependent(DiffussionParamsTimeIndependent),
    DiffussionSolverTimeDependent(DiffussionParamsTimeDependent),
    None
}

#[derive(Debug)]
pub struct NoSolver();

impl DiffEquationSolver for NoSolver {

    fn solve(&mut self, _time_step: f64) -> Result<Vec<f64>, crate::Error> {
        Ok(vec![])
    }
}