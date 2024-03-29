// Local dependencies
use crate::Error;

// External dependencies
use std::fmt::Debug;

/// # General Information
///
/// A struct that implements DiffEquationSolver is implied to contain all needed information for a certain ODE/PDE to be solved. Therefore, a function to solve the
/// equation needs to be present in such a structure.
///
pub trait DiffEquationSolver: Debug {
    /// # General Information
    ///
    /// solve returns a vector of values representing the solution of an equation at a given collection of nodes provided by the user at the creation of an
    /// instance of a solver.
    ///
    /// # Parameters
    ///
    /// * `&self` - An instance of an ODE/PDE solver.
    /// * `time_step` - Optional for time independent methods, but important for others to move forward the solution.
    ///
    fn solve(&mut self, time_step: f64) -> Result<Vec<f64>, Error>;
}