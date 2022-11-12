use crate::solvers::fem::basis::single_variable::{
    linear_basis::LinearBasis, polynomials_1d::FirstDegreePolynomial, Differentiable1D, Function1D,
};
use crate::solvers::{DiffEquationSolver, matrix_solver};
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
/// * `internal_state` - The state of every internal point at time t. Struct has to be mutable.
/// * `mesh` - A vector of floats representing a line.
/// * `mu` - First ot two needed constants.
/// * `b` - Second of two needed constants.
///
pub struct DiffussionSolverTimeDependent {
    boundary_conditions: [f64; 2],
    initial_conditions: Vec<f64>,
    internal_state: Vec<f64>,
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
            internal_state: initial_conditions.clone(),
            initial_conditions,
            mesh,
            mu,
            b,
        })
    }

    /// # General Information
    /// 
    /// Compĺete integration of mass matrix and vector b to create system Mx = b.
    /// Note that corners of the linear system of equations are treated differently since, normally, there's one less addition to make.
    /// 
    /// # Parameters
    /// 
    /// * `&self` - A reference to itself to use parameters b, mu and mesh.
    /// * `gauss_step` - Amount of nodes to compute for integration.
    /// * `time_step` - How much to advance the solution.
    /// 
    pub fn gauss_legendre_integration(&self, gauss_step: usize, time_step: f64) -> (Array2<f64>,Array1<f64>) {
        
        // First generate the basis
        let linear_basis = LinearBasis::new(&self.mesh).unwrap();
        let basis_len = linear_basis.basis.len();

        // initialize mass matrix (internal, no boundaries included)
        let mut matrix = ndarray::Array::from_elem((basis_len - 2, basis_len - 2), 0_f64);
        // initialize vector b (internal, no boundaries included)
        let mut b = ndarray::Array::from_elem(basis_len - 2, 0_f64);

        for i in 1..(basis_len - 1) {
            // Now we calculate every integral in the equation.
            // One needs to be careful regarding the boundary of the matrix.
            // Obtain every integral. Later on integrals are assigned to the corresponding matrx or vector element.
            let derivative_phi = linear_basis.basis[i].differentiate();
            // replaced by boundary condition for basis[n-1] in vector
            let derivative_phi_next = linear_basis.basis[i+1].differentiate();
            // replaced by boundary condition for basis[0] in vector
            let derivative_phi_prev = linear_basis.basis[i-1].differentiate();

            // Transform intervals from -1,1 to [ai,bi]
            let transform_function_prev = FirstDegreePolynomial::transformation_from_m1_p1(
                self.mesh[i-1],
                self.mesh[i],
            );
            let transform_function_next = FirstDegreePolynomial::transformation_from_m1_p1(
                self.mesh[i],
                self.mesh[i+1],
            );
            let transform_function_square =
                FirstDegreePolynomial::transformation_from_m1_p1(
                    self.mesh[i-1],
                    self.mesh[i+1],
                );
    
            // transform functions' derivatives
            let derivative_t_prev = transform_function_prev.differentiate();
            let derivative_t_next = transform_function_next.differentiate();
            let derivative_t_square = transform_function_square.differentiate();
            
            // initialize all integral approximations
            // derivatives integral. Of the form <phi_j',phi_i'>
            let mut integral_prev_approximation_prime = 0_f64;
            let mut integral_next_approximation_prime = 0_f64;
            let mut integral_square_approximation_prime = 0_f64;
            // half derivative integral. Of the form <phi_j,phi_i'>
            let mut integral_prev_approximation_half = 0_f64;
            let mut integral_next_approximation_half = 0_f64;
            let mut integral_square_approximation_half = 0_f64;
            // normal integrals. Of the form <phi_j,phi_i>
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
                    linear_basis.basis[i].evaluate(translated_point_prev) *
                    linear_basis.basis[i-1].evaluate(translated_point_prev) * derivative_t_prev.evaluate(x) * w;
                // dot product <phi_1,phi_1>
                integral_square_approximation_mass +=
                    linear_basis.basis[i].evaluate(translated_point_square).powf(2_f64) *
                    derivative_t_square.evaluate(x) * w;
                // dot product <phi_1,phi_2>
                integral_next_approximation_mass +=
                    linear_basis.basis[i].evaluate(translated_point_next) *
                    linear_basis.basis[i+1].evaluate(translated_point_next) * derivative_t_next.evaluate(x) * w;
                
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
                
                // Half derivative integrals
                // integral <phi_1,phi_0'>
                integral_prev_approximation_half += 
                linear_basis.basis[i].evaluate(translated_point_prev) *
                derivative_phi_prev.evaluate(translated_point_prev) * derivative_t_prev.evaluate(x) * w;
                // integral <phi_1,phi_1'>
                integral_square_approximation_half += 
                linear_basis.basis[i].evaluate(translated_point_square) *
                derivative_phi.evaluate(translated_point_square) * derivative_t_square.evaluate(x) * w;
                // integral <phi_1,phi_2'>
                integral_next_approximation_half += 
                linear_basis.basis[i].evaluate(translated_point_next) *
                derivative_phi_next.evaluate(translated_point_next) * derivative_t_next.evaluate(x) * w;
            }

            if i == 1 {

                // matrix indices are one unit less than original unit because of initialization
                // last two approximations to mass matrix are put inside final matrix
                matrix[[i-1,0]] = integral_square_approximation_mass;
                matrix[[i-1,1]] = integral_next_approximation_mass;

                // b indices also have the same delay as the matrix
                // left-side boundary condition is added to b
                b[i-1] += -integral_prev_approximation_mass * self.boundary_conditions[0];

                // add the rest of b[i-1] elements
                b[i-1] +=
                    // Add u_ni * <phi_i,phi_j>
                    // supposes dirichlet boundary conditions ----------------
                    self.boundary_conditions[0] * integral_prev_approximation_mass +
                    // supposes dirichlet boundary conditions ------------
                    self.internal_state[i-1] * integral_square_approximation_mass +
                    self.internal_state[i] * integral_next_approximation_mass -
                    // Add - t * mu * u_ni * <phi_i',phi_j'>
                    (self.boundary_conditions[0] * integral_prev_approximation_prime +
                    self.internal_state[i-1] * integral_square_approximation_prime +
                    self.internal_state[i] * integral_next_approximation_prime) * time_step * self.mu - 
                    // Add -t * b * u_ni * <phi_i',phi_j>
                    (self.boundary_conditions[0] * integral_prev_approximation_half +
                    self.internal_state[i-1] * integral_square_approximation_half +
                    self.internal_state[i] * integral_next_approximation_half) * time_step * self.b;

            } else if i == basis_len - 2 {

                matrix[[i-1,basis_len-4]] = integral_prev_approximation_mass;
                matrix[[i-1,basis_len-3]] = integral_square_approximation_mass;

                //right-side boundary condition is addded to b
                b[i-1] += - integral_next_approximation_mass * self.boundary_conditions[1];

                // add the rest of b[i-1] elements
                b[i-1] +=
                    // Add u_ni * <phi_i,phi_j>
                    // supposes dirichlet boundary conditions ----------------
                    self.internal_state[i-2] * integral_prev_approximation_mass +
                    // supposes dirichlet boundary conditions ------------
                    self.internal_state[i-1] * integral_square_approximation_mass +
                    self.boundary_conditions[1] * integral_next_approximation_mass -
                    // Add - t * mu * u_ni * <phi_i',phi_j'>
                    (self.internal_state[i-2] * integral_prev_approximation_prime +
                    self.internal_state[i-1] * integral_square_approximation_prime +
                    self.boundary_conditions[1] * integral_next_approximation_prime) * time_step * self.mu - 
                    // Add -t * b * u_ni * <phi_i',phi_j>
                    (self.internal_state[i-2] * integral_prev_approximation_half +
                    self.internal_state[i-1] * integral_square_approximation_half +
                    self.boundary_conditions[1] * integral_next_approximation_half) * time_step * self.b;

            } else {

                matrix[[i-1,i-2]] = integral_prev_approximation_mass;
                matrix[[i-1,i-1]] = integral_square_approximation_mass;
                matrix[[i-1,i]] = integral_next_approximation_mass;

                // add the rest of b[i-1] elements
                // the integrals of the same structure are added, since it works like matrix multiplication to obtain a vector
                b[i-1] +=
                    // Add u_ni * <phi_i,phi_j> 
                    self.internal_state[i-2] * integral_prev_approximation_mass +
                    self.internal_state[i-1] * integral_square_approximation_mass +
                    self.internal_state[i] * integral_next_approximation_mass -
                    // Add - t * mu * u_ni * <phi_i',phi_j'>
                    (self.internal_state[i-2] * integral_prev_approximation_prime +
                    self.internal_state[i-1] * integral_square_approximation_prime +
                    self.internal_state[i] * integral_next_approximation_prime) * time_step * self.mu - 
                    // Add -t * b * u_ni * <phi_i',phi_j>
                    (self.internal_state[i-2] * integral_prev_approximation_half +
                    self.internal_state[i-1] * integral_square_approximation_half +
                    self.internal_state[i] * integral_next_approximation_half) * time_step * self.b;
            
            }
        }

        (matrix,b)

    }
}

impl DiffEquationSolver for DiffussionSolverTimeDependent {

    fn solve(&mut self, integration_step: usize, time_step: f64) -> Result<Vec<f64>, Error> {
        
        let (a, b) = self.gauss_legendre_integration(integration_step, time_step);

        println!("A: {:?} and b: {:?}",a,b);

        let mut res = matrix_solver::solve_by_thomas(&a, &b)?;
        
        self.internal_state = res[1..=b.len()].to_vec();

        // Adding boundary condition values
        res[0] = self.boundary_conditions[0];
        res[b.len() + 1] = self.boundary_conditions[1];


        Ok(res)

    }
}
#[cfg(test)]
mod tests {

    #[test]
    fn test_matrix_and_vector_values_4p() {

    }
}
