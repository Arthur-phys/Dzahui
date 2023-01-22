// Internal dependencies
use crate::solvers::fem::basis::single_variable::{
    linear_basis::LinearBasis, polynomials_1d::{FirstDegreePolynomial, SecondDegreePolynomial}, Differentiable1D, Function1D,
};
use crate::solvers::{quadrature::gauss_legendre, matrix_solver, solver_trait::DiffEquationSolver};
use crate::Error;

// External dependencies
use ndarray::{Array1, Array2};

