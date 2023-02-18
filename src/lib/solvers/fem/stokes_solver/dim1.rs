use std::fmt::Debug;

// Internal dependencies
use crate::solvers::fem::basis::single_variable::{
    linear_basis::LinearBasis, polynomials_1d::FirstDegreePolynomial, Differentiable1D, Function1D,
};
use crate::solvers::{quadrature::gauss_legendre, matrix_solver, solver_trait::DiffEquationSolver};
use crate::Error;

// External dependencies
use ndarray::{Array1, Array2};

/// # General Information
/// 
/// Parameters needed for solving Stokes equation in 1d.
/// If one of it's properties is not set, it will default to zero.
/// Boundary conditions accepted are only Dirichlet for now.
/// 
/// # Parameters
/// 
/// * `rho` - Constant density
/// * `pressure` - Pressure [0] at index [1]
/// * `boundary_condition_pressure` - Pressure boundary condition
/// 
pub struct StokesParams1D {
    pub rho: f64,
    pub hydrostatic_pressure: f64,
    pub force_function: Box<dyn Fn(f64) -> f64>,
}


impl Default for StokesParams1D {
    fn default() -> Self {
        Self {
            rho: 0_f64,
            hydrostatic_pressure: 0_f64,
            force_function: Box::new(|x| x)
        }
    }
}

impl Debug for StokesParams1D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ff = &self.force_function;
        let eval = ff(0_f64);
        let content = format!("{{ rho: {},\nhydrostatic_pressure: {},\n force_function: f(0) -> {} }}", self.rho, self.hydrostatic_pressure,eval);
        write!(f, "{}", content)
    }
}

#[derive(Debug)]
/// # General Information
///
/// A Stokes solver 1d abstracts the equation: "(1/ρ)p_x = f" with constant velocity and hydrostatic pressure "ρ"
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
pub struct StokesSolver1D {
    pub(crate) stiffness_matrix: Array2<f64>,
    pub(crate) b_vector: Array1<f64>,
    pub gauss_step: usize,
    pub hydrostatic_pressure: f64,
    pub rho: f64,
}

impl StokesSolver1D {

    pub fn new(params: &StokesParams1D, mesh: Vec<f64>, gauss_step: usize) -> Result<Self,Error> {

        let (stiffness_matrix, b_vector) = Self::gauss_legendre_integration(
            params.rho,
            params.hydrostatic_pressure,
            &mesh,
            gauss_step,
            &params.force_function
        )?;
        Ok(Self {
            stiffness_matrix,
            gauss_step,
            b_vector,
            hydrostatic_pressure: params.hydrostatic_pressure,
            rho: params.rho
        })

    }

    pub fn gauss_legendre_integration(rho: f64, hydrostatic_pressure: f64, mesh: &Vec<f64>, gauss_step: usize, function: &Box<dyn Fn(f64) -> f64>) -> Result<(Array2<f64>, Array1<f64>),Error> {

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
        
        let derivative_phi_0 = basis.basis[0].differentiate()?;
        let derivative_phi_1 = basis.basis[1].differentiate()?;

        let transform_function_square_0 =
            FirstDegreePolynomial::transformation_from_m1_p1(
                mesh[0],
                mesh[1],
            );
        let derivative_t_square_0 = transform_function_square_0.differentiate()?;

        let mut integral_0_approximation = 0_f64;
        let mut integral_0_next_approximation = 0_f64;
        let mut b_first_integral_approximation = 0_f64;


        for j in 1..gauss_step {

            // Obtaining arccos(node) and weight
            let (theta, w) = gauss_legendre::quad_pair(gauss_step, j)?;
            let x = theta.cos();

            let translated_0 = transform_function_square_0.evaluate(x);

            integral_0_approximation += basis.basis[0].evaluate(translated_0) * 
                derivative_phi_0.evaluate(translated_0) * 
                derivative_t_square_0.evaluate(x) * w;
            
            integral_0_next_approximation += basis.basis[0].evaluate(translated_0) * 
            derivative_phi_1.evaluate(translated_0) * 
            derivative_t_square_0.evaluate(x) * w;

            b_first_integral_approximation += rho * function(translated_0) *
            basis.basis[0].evaluate(translated_0) *
            derivative_t_square_0.evaluate(x) * w;

        }

        stiffness_matrix[[0, 0]] = integral_0_approximation;
        stiffness_matrix[[0, 1]] = integral_0_next_approximation;
        stiffness_matrix[[basis_len-1,basis_len-1]] = 1_f64;
        b_vector[0] = b_first_integral_approximation;
        b_vector[basis_len - 1] = hydrostatic_pressure;

        Ok((stiffness_matrix, b_vector))

    }
}

impl DiffEquationSolver for StokesSolver1D {
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
    use crate::StokesParams;

    use super::{StokesSolver1D,DiffEquationSolver};

    #[test]
    fn regular_mesh_matrix_4p_nav() {
        
        let params = StokesParams::normal_1d().force_function(Box::new(|_| 10_f64))
            .hydrostatic_pressure(1_f64).density(1_f64).build();

        let mut eq = StokesSolver1D::new(&params, vec![0_f64,0.333,0.666,1_f64], 150).unwrap();

        assert!(eq.stiffness_matrix[[0, 0]] <= -0.4 && eq.stiffness_matrix[[0, 0]] >= -0.6);
        assert!(eq.stiffness_matrix[[0, 1]] <= 0.6 && eq.stiffness_matrix[[0, 1]] >= 0.4);
        assert!(eq.stiffness_matrix[[1, 0]] <= -0.4 && eq.stiffness_matrix[[1, 0]] >= -0.6);
        assert!(eq.stiffness_matrix[[1, 1]] <= 0.1 && eq.stiffness_matrix[[1, 1]] >= -0.1);
        assert!(eq.stiffness_matrix[[1, 2]] <= 0.6 && eq.stiffness_matrix[[1, 2]] >= 0.4);
        assert!(eq.stiffness_matrix[[2, 1]] <= -0.4 && eq.stiffness_matrix[[2, 1]] >= -0.6);
        assert!(eq.stiffness_matrix[[2, 2]] <= 0.1 && eq.stiffness_matrix[[2, 2]] >= -0.1);
        assert!(eq.stiffness_matrix[[2, 3]] <= 0.6 && eq.stiffness_matrix[[2, 3]] >= 0.4);
        assert!(eq.stiffness_matrix[[3, 2]] == 0_f64);
        assert!(eq.stiffness_matrix[[3, 3]] == 1_f64);
    
        println!("{:?}",eq.b_vector);
        assert!(eq.b_vector[[0]] <= 1.75 && eq.b_vector[[0]] >= 1.55);       
        assert!(eq.b_vector[[1]] <= 3.45 && eq.b_vector[[1]] >= 3.25);       
        assert!(eq.b_vector[[2]] <= 3.45 && eq.b_vector[[2]] >= 3.25);       
        assert!(eq.b_vector[[3]] == 1_f64);
        
        let solution = eq.solve(0_f64).unwrap();

        assert!(solution[0] <= -8.9 && solution[0] >= -9.1);
        assert!(solution[1] <= -5.5 && solution[1] >= -5.7);
        assert!(solution[2] <= -2.2 && solution[2] >= -2.4);

    }
}