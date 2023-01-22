use std::fmt::Debug;

// Internal dependencies
use crate::solvers::fem::basis::single_variable::{
    linear_basis::LinearBasis, polynomials_1d::{FirstDegreePolynomial}, Differentiable1D, Function1D,
};
use crate::solvers::{quadrature::gauss_legendre, matrix_solver, solver_trait::DiffEquationSolver};
use crate::Error;

// External dependencies
use ndarray::{Array1, Array2};

/// # General Information
/// 
/// Parameters needed for solving Navier-Stokes equation in 1d with time-independence.
/// If one of it's properties is not set, it will default to zero.
/// Boundary conditions accepted are only Dirichlet for now.
/// 
/// # Parameters
/// 
/// * `rho` - Constant density
/// * `speed` - Constant speed
/// * `boundary_condition_pressure` - Pressure boundary condition
/// 
pub struct NavierStokesParams1DTimeIndependent {
    pub rho: f64,
    pub speed: f64,
    pub force_function: Box<dyn Fn(f64) -> f64>,
}

impl Default for NavierStokesParams1DTimeIndependent {
    fn default() -> Self {
        Self {
            rho: 0_f64,
            speed: 0_f64,
            force_function: Box::new(|x| x)
        }
    }
}

impl Debug for NavierStokesParams1DTimeIndependent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ff = &self.force_function;
        let eval = ff(0_f64);
        let content = format!("{{ rho: {},\nspeed: {},\n force_function: f(0) -> {} }}", self.rho, self.speed,eval);
        write!(f, "{}", content)
    }
}

#[derive(Debug)]
/// # General Information
///
/// A Navier-Stokes solver with time-independence abstracts the equation: "(1/ρ)p_x = f" with constant velocity and contains boundary conditions along with "ρ"
///
/// # Fields
///
/// * `boundary_condition_pressure` - Original boundary conditions (Only Dirichlet is supported for now).
/// * `stiffness_matrix` - Left-side matrix of resulting discrete equation.
/// * `b_vector` - Right-side vector of the resulting discrete equation.
/// * `gauss_step` - Precision of quadrature.
/// * `speed` - Constant speed.
/// * `rho` - Constant density.
///
pub struct NavierStokesSolver1DTimeIndependent {
    pub(crate) stiffness_matrix: Array2<f64>,
    pub(crate) b_vector: Array1<f64>,
    pub gauss_step: usize,
    pub speed: f64,
    pub rho: f64,
}

impl NavierStokesSolver1DTimeIndependent {

    pub fn new(params: &NavierStokesParams1DTimeIndependent, mesh: Vec<f64>, gauss_step: usize) -> Result<Self,Error> {

        let (stiffness_matrix, b_vector) = Self::gauss_legendre_integration(
            params.rho,
            &mesh,
            gauss_step,
            &params.force_function
        )?;
        Ok(Self {
            stiffness_matrix,
            gauss_step,
            b_vector,
            speed: params.speed,
            rho: params.rho
        })

    }

    pub fn gauss_legendre_integration(rho: f64, mesh: &Vec<f64>, gauss_step: usize, function: &Box<dyn Fn(f64) -> f64>) -> Result<(Array2<f64>, Array1<f64>),Error> {

        let basis = LinearBasis::new(mesh)?;
        let basis_len = basis.basis.len();

        let mut stiffness_matrix =
            ndarray::Array::from_elem((basis_len, basis_len), 0_f64);

        let mut b_vector = Array1::from_elem(basis_len, 0_f64);


        for i in 1..(basis_len - 1) {

            let derivative_phi = basis.basis[i].differentiate()?;

            let transform_function_prev = FirstDegreePolynomial::transformation_from_m1_p1(
                mesh[i - 1],
                mesh[i],
            );
            let transform_function_next = FirstDegreePolynomial::transformation_from_m1_p1(
                mesh[i],
                mesh[i + 1],
            );
            let transform_function_square =
                FirstDegreePolynomial::transformation_from_m1_p1(
                    mesh[i - 1],
                    mesh[i + 1],
                );

            let derivative_t_prev = transform_function_prev.differentiate()?;
            let derivative_t_next = transform_function_next.differentiate()?;
            let derivative_t_square = transform_function_square.differentiate()?;

            let derivative_prev = basis.basis[i - 1].differentiate()?;
            let derivative_next = basis.basis[i + 1].differentiate()?;

            let mut integral_prev_approximation = 0_f64;
            let mut integral_next_approximation = 0_f64;
            let mut integral_square_approximation = 0_f64;
            let mut b_integral_approximation = 0_f64;

            // integrate
            for j in 1..gauss_step {
                // Obtaining arccos(node) and weight
                let (theta, w) = gauss_legendre::quad_pair(gauss_step, j)?;
                let x = theta.cos();

                // translated from -1,1
                let translated_point_prev = transform_function_prev.evaluate(x);
                let translated_point_next = transform_function_next.evaluate(x);
                let translated_point_square = transform_function_square.evaluate(x);

                integral_prev_approximation +=
                    basis.basis[i].evaluate(translated_point_prev)*
                    derivative_prev.evaluate(translated_point_prev)*
                    derivative_t_prev.evaluate(x)*
                    w;
                integral_next_approximation +=
                    basis.basis[i].evaluate(translated_point_next)*
                    derivative_next.evaluate(translated_point_next)*
                    derivative_t_next.evaluate(x)*
                    w;
                integral_square_approximation +=
                    basis.basis[i].evaluate(translated_point_square)*
                    derivative_phi.evaluate(translated_point_square)*
                    derivative_t_square.evaluate(x)*
                    w;
                b_integral_approximation += rho*
                    function(translated_point_square)*
                    basis.basis[i].evaluate(translated_point_square)*
                    derivative_t_square.evaluate(x)*
                    w;
            }

            stiffness_matrix[[i, i]] = integral_square_approximation;
            stiffness_matrix[[i, i - 1]] = integral_prev_approximation;
            stiffness_matrix[[i, i + 1]] = integral_next_approximation;
            b_vector[i] = b_integral_approximation;
        
        }

        Ok((stiffness_matrix, b_vector))

    }
}

impl DiffEquationSolver for NavierStokesSolver1DTimeIndependent {
    /// # Specific implementation
    ///
    /// Solving starts by obtaining stiffness matrix and vector b (Ax=b).
    /// Then both are used inside function `solve_by_thomas` to obtain the result vector.
    ///
    fn solve(&mut self, _time_step: f64) -> Result<Vec<f64>, Error> {

        let res = matrix_solver::solve_by_thomas(&self.stiffness_matrix, &self.b_vector)?;

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    
}