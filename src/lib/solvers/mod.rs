pub mod euler;
pub mod fem;
pub mod linear_solver;
pub mod quadrature;

use std::fmt::Debug;

use crate::Error;

pub use fem::*;
pub use fem::Solver;


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
    /// * &self - An instance of an ODE/PDE solver.
    ///
    fn solve(&self) -> Result<Vec<f64>, Error>;
}
