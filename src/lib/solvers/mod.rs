pub mod linear_solver;
pub mod quadrature;
pub mod euler;
pub mod fem;

use std::fmt::Debug;

pub use fem::fem_ode::*;
pub use fem::Solver;
use ndarray::Array1;

use crate::Error;

/// # General Information
/// 
/// A struct that implements DiffEquationSolver is implied to contain all needed information for a certain ODE/PDE to be solved. Therefore, a function to solve the
/// equation needs to be present in such a structure.
/// This trait **does not** consider time to be within the variables to be solved for. For that case, refer to `TimeDiffEquationSolver`.
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
    fn solve(&self) -> Result<Array1<f64>, Error>;
}
