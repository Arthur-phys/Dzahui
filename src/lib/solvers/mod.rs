pub mod euler;
pub mod fem;
pub mod linear_solver;
pub mod quadrature;

pub use fem::fem_ode::*;

use std::ops::Index;
use crate::Error;


/// # General Information
/// 
/// A struct that implements DiffEquationSolver is implied to contain all needed information for a certain ODE/PDE to be solved. Therefore, a function to solve the
/// equation needs to be present in such a structure.
/// This trait **does not** consider time to be within the variables to be solved for. For that case, refer to `TimeDiffEquationSolver`.
/// 
pub trait DiffEquationSolver<A>
    where A: Index<usize> + IntoIterator, A::Item: Into<f64> {
    
    /// # General Information
    /// 
    /// solve returns a vector of values representing the solution of an equation at a given collection of nodes provided by the user at the creation of an
    /// instance of a solver.
    /// 
    /// # Parameters
    /// 
    /// * &self - An instance of an ODE/PDE solver.
    ///
    fn solve(&self) -> Result<A, Error>;
}

/// # General Information
/// 
/// A struct that implements TimeDiffEquationSolver is implied to contain all needed information for a certain time-dependant ODE/PDE to be solved.
/// A time dependant ODE needs to have a time-step assigned to the function that is to solve the problem, that's why a delta time is accepted by the main function
/// of the trait.
/// 
pub trait TimeDiffEquationSolver<A>
    where A: Index<usize> + IntoIterator, A::Item: Into<f64> {
    
    /// # General Information
    /// 
    /// do_step returns a vector of values representing the solution at a given time and collection of nodes provided by the user at the creation of an instance
    /// of a time-dependant solver.
    /// 
    /// # Parameters
    /// 
    /// * &self - An instance of an ODE/PDE solver.
    /// 
    fn do_step(&self) -> A;
}
