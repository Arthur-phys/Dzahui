// Module definition
pub mod basis;
pub mod diffusion_solver;

/// # General Information
///
/// An enum representing every equation implemented by this library.
/// Used as a way of representing the equation without having to create an instance of a solver of it. May be deprecated in the future in favor of a intercative
/// approach from window interface.
///
/// # Arms
///
/// * `DiffussionSolverTimeIndependent` - Diffusion equation solver representation. Currently accepting parameters, may not be the case in the future.
/// * `DiffussionSolverTimeDependent` - Diffusion equation solver with time derivative representation. Currently accepting parameters, may not be the case in the future.
/// * `NavierStokes1DSolver` - Navier Stokes in 1D solver representation.
/// * `NavierStokes2DSolver` - Navier Stokes in 2D solver representation.
/// * `None` - Purelly visuallization of mesh in simulation. No equation attached.
///
#[derive(Debug)]
pub enum Solver {
    DiffussionSolverTimeIndependent([f64; 2], f64, f64),
    DiffussionSolverTimeDependent([f64; 2], Vec<f64>, f64, f64),
    None
}
