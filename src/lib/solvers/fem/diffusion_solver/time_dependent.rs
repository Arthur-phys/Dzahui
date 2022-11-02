use crate::solvers::fem::basis::single_variable::{
    linear_basis::LinearBasis, polynomials_1d::FirstDegreePolynomial, Differentiable1D, Function1D,
};
use crate::solvers::DiffEquationSolver;
use crate::solvers::quadrature::GaussLegendreQuadrature;
use crate::Error;

use ndarray::{Array, Array1, Ix1, Ix2};

#[derive(Debug)]
/// # General Information
///
/// A diffusion solver with time-independence abstracts the equation: "- μu_xx + bu_x = 0" and contains boundary conditions along with mesh, "b" and "μ"
///
/// # Fields
///
/// * `boundary_conditions` - Original boundary conditions (Only dirichlet is supported for now, Neumann is being worked on).
/// * `mesh` - A vector of floats representing a line.
/// * `mu` - First ot two needed constants.
/// * `b` - Second of two needed constants.
///
pub struct DiffussionSolverTimeDependent {
    boundary_conditions: [f64; 2],
    mesh: Vec<f64>,
    mu: f64,
    b: f64,
}

impl DiffussionSolverTimeDependent {
    /// Creates new instance
    pub fn new(boundary_conditions: [f64; 2], mesh: Vec<f64>, mu: f64, b: f64) -> Self {
        Self {
            boundary_conditions,
            mesh,
            mu,
            b,
        }
    }
}