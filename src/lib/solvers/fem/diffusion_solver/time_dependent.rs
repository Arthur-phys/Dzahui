use crate::solvers::fem::basis::single_variable::{
    linear_basis::LinearBasis, polynomials_1d::FirstDegreePolynomial, Differentiable1D, Function1D,
};
use crate::solvers::DiffEquationSolver;
use crate::Error;
use crate::solvers::quadrature::gauss_legendre;

use ndarray::{Array1, Array2};

#[derive(Debug)]
/// # General Information
///
/// A diffusion solver with time-dependence abstracts the equation: "u_t - μu_xx + bu_x = 0" and contains boundary conditions,
/// initial conditions, mesh, "b" and "μ".
///
/// # Fields
///
/// * `boundary_conditions` - Boundary conditions (Only dirichlet is supported for now, Neumann is being worked on).
/// * `initial_conditions` - Every internal point needs an initial condition to advance the solution in time.
/// * `mesh` - A vector of floats representing a line.
/// * `mu` - First ot two needed constants.
/// * `b` - Second of two needed constants.
///
pub struct DiffussionSolverTimeDependent {
    boundary_conditions: [f64; 2],
    initial_conditions: Vec<f64>,
    mesh: Vec<f64>,
    mu: f64,
    b: f64,
}

impl DiffussionSolverTimeDependent {
    /// Creates new instance checking initial conditions are the size they should be.
    pub fn new(boundary_conditions: [f64; 2], initial_conditions: Vec<f64>, mesh: Vec<f64>, mu: f64, b: f64) -> Result<Self,Error> {
        if initial_conditions.len() != mesh.len() - 2 {
            return Err(Error::WrongDims)
        }
        Ok(Self {
            boundary_conditions,
            initial_conditions,
            mesh,
            mu,
            b,
        })
    }

    pub fn gauss_legendre_integration(&self, gauss_step: usize, time_step: f64) -> (Array2<f64>,Array1<f64>) {
        
        // First generate the basis
        let linear_basis = LinearBasis::new(&self.mesh).unwrap();
        let basis_len = linear_basis.basis.len();

        // initialize stiffness matrix (internal, no boundaries included)
        let mut matrix = ndarray::Array::from_elem((basis_len - 2, basis_len - 2), 0_f64);
        // initialize vector b (internal, no boundaries included)
        let mut b = ndarray::Array::from_elem((basis_len - 2), 0_f64);

        // Now we calculate every integral in the equation.
        // One needs to be careful regarding the boundary of the matrix.
        // Obtain row 0 for matrix (corresponds to phi_1 in basis) and element 0 for b.
        let derivative_phi = linear_basis.basis[1].differentiate();
        let derivative_phi_next = linear_basis.basis[2].differentiate();
        //boundary condition in basis[0]
        let derivative_phi_prev = linear_basis.basis[0].differentiate();

        // Transform intervals from -1,1 to [ai,bi]
        let transform_function_prev = FirstDegreePolynomial::transformation_from_m1_p1(
            self.mesh[0],
            self.mesh[1],
        );
        let transform_function_next = FirstDegreePolynomial::transformation_from_m1_p1(
            self.mesh[1],
            self.mesh[2],
        );
        let transform_function_square =
            FirstDegreePolynomial::transformation_from_m1_p1(
                self.mesh[0],
                self.mesh[2],
            );

        // transform functions' derivatives
        let derivative_t_prev = transform_function_prev.differentiate();
        let derivative_t_next = transform_function_next.differentiate();
        let derivative_t_square = transform_function_square.differentiate();

        // initialize b integral approximations
        // derivatives integral. Of the form <phi_j',phi_i'>
        let mut integral_prev_approximation_prime = 0_f64;
        let mut integral_next_approximation_prime = 0_f64;
        let mut integral_square_approximation_prime = 0_f64;
        // half derivative integral. Of the form <phi_j,phi_i'>
        let mut integral_prev_approximation_half = 0_f64;
        let mut integral_next_approximation_half = 0_f64;
        let mut integral_square_approximation_half = 0_f64;
        // initialize mass matrix
        let mut integral_prev_approximation_mass = 0_f64;
        let mut integral_next_approximation_mass = 0_f64;
        let mut integral_square_approximation_mass = 0_f64;

        //integrate:
        for j in 1..gauss_step {

            // Obtaining arccos(node) and weight
            let (theta, w) = gauss_legendre::quad_pair(gauss_step, j);
            let x = theta.cos();

            // translated from -1,1
            // x is evaluated inside phi_i function according to change of variable rule
            let translated_point_prev = transform_function_prev.evaluate(x);
            let translated_point_next = transform_function_next.evaluate(x);
            let translated_point_square = transform_function_square.evaluate(x);

            // Dot product integrals
            // dot product <phi_1,phi_0>
            integral_prev_approximation_mass += 
                linear_basis.basis[1].evaluate(translated_point_prev) *
                linear_basis.basis[0].evaluate(translated_point_prev) * derivative_t_prev.evaluate(x) * w;
            // dot product <phi_1,phi_1>
            integral_square_approximation_mass +=
                linear_basis.basis[1].evaluate(translated_point_square).powf(2_f64) *
                derivative_t_square.evaluate(x) * w;
            // dot product <phi_1,phi_2>
            integral_next_approximation_mass +=
                linear_basis.basis[1].evaluate(translated_point_next) *
                linear_basis.basis[2].evaluate(translated_point_next) * derivative_t_next.evaluate(x) * w;
            
            // Derivative integrals
            // integral <phi_1',phi_0'>
            integral_prev_approximation_prime +=
            derivative_phi.evaluate(translated_point_prev) *
            derivative_phi_prev.evaluate(translated_point_prev) * derivative_t_prev.evaluate(x) * w;
            // integral <phi_1',phi_1'>
            integral_square_approximation_prime +=
            derivative_phi.evaluate(translated_point_square).powf(2_f64) *
            derivative_t_square.evaluate(x) * w;
            // integral <phi_1',phi_2'>
            integral_next_approximation_prime +=
            derivative_phi.evaluate(translated_point_next) *
            derivative_phi_next.evaluate(translated_point_next) * derivative_t_next.evaluate(x) * w;
            
            // <---------------------------------- keep going from here ------------------------------------------>
            // Half derivative integrals
            // integral <phi_1,phi_0'>
            // integral <phi_1,phi_1'>
            // integral <phi_1,phi_2'>

        }

        // last two approximations to mass matrix are put inside final matrix
        matrix[[0,0]] = integral_square_approximation_mass;
        matrix[[0,1]] = integral_next_approximation_mass;
        // left-side boundary condition is added to b
        b[0] += -integral_prev_approximation_mass * self.boundary_conditions[0];

        // Every non-boundary element (skip 0, basis_len - 3 from matrix an b, which correspond to phi_1 and phi_(basis_len-2))
        for i in 2..(basis_len - 2) {

            let derivative_phi = linear_basis.basis[i].differentiate();
        }
        
        todo!();
    }
}

impl DiffEquationSolver for DiffussionSolverTimeDependent {

    fn solve(&self) -> Result<Vec<f64>, Error> {
        todo!();
    }
}
