use crate::solvers::fem::basis::single_variable::{
    linear_basis::LinearBasis, polynomials_1d::FirstDegreePolynomial, Differentiable1D, Function1D,
};
use crate::solvers::utils;
use crate::solvers::{DiffEquationSolver, matrix_solver};
use crate::Error;
use crate::solvers::quadrature::gauss_legendre;

use ndarray::{Array1, Array2};

#[derive(Debug)]
/// # General Information
///
/// A diffusion solver with time-dependence abstracts the equation: "u_t - μu_xx + bu_x = 0" and contains boundary conditions,
/// initial conditions, mesh, "stiffness_matrix" and "μ".
///
/// # Fields
///
/// * `boundary_conditions` - Boundary conditions (Only dirichlet is supported for now, Neumann is being worked on).
/// * `initial_conditions` - Every internal point needs an initial condition to advance the solution in time.
/// * `internal_state` - The state of every internal point at time t. Struct has to be mutable.
/// * `mesh` - A vector of floats representing a line.
/// * `mu` - First ot two needed constants.
/// * `stiffness_matrix` - Second of two needed constants.
///
pub struct DiffussionSolverTimeDependent {
    pub boundary_conditions: [f64; 2],
    stiffness_matrix: Array2<f64>,
    pub initial_conditions: Vec<f64>,
    mass_matrix: Array2<f64>,
    pub integration_step: usize,
    state: Array1<f64>,
    mesh: Vec<f64>,
    pub mu: f64,
    pub b: f64,
}

impl DiffussionSolverTimeDependent {
    /// Creates new instance checking initial conditions are the size they should be.
    pub fn new(boundary_conditions: [f64; 2], initial_conditions: Vec<f64>, integration_step: usize, mesh: Vec<f64>, mu: f64, b: f64) -> Result<Self,Error> {
        if initial_conditions.len() != mesh.len() - 2 {
            return Err(Error::WrongDims)
        }

        // obtain general initial state
        let mut state = vec![0_f64;mesh.len()];
        state[0] = boundary_conditions[0];
        state[mesh.len() - 1] = boundary_conditions[1]; 
        for i in 1..(mesh.len() - 1) {
            state[i] = initial_conditions[i-1];
        }

        let state = Array1::from_vec(state);

        let (mass_matrix, stiffness_matrix) = Self::gauss_legendre_integration(
                mu, b, boundary_conditions, &mesh, integration_step);

        // obtain matrices

        Ok(Self {
            boundary_conditions,
            initial_conditions,
            stiffness_matrix,
            integration_step,
            mass_matrix,
            state,
            mesh,
            mu,
            b,
        })
    }

    /// # General Information
    /// 
    /// Compĺete integration of mass mass_matrix and vector stiffness_matrix to create system Mx = stiffness_matrix.
    /// Note that corners of the linear system of equations are treated differently since, normally, there's one less addition to make.
    /// 
    /// # Parameters
    /// 
    /// * `&self` - A reference to itself to use parameters stiffness_matrix, mu and mesh.
    /// * `gauss_step` - Amount of nodes to compute for integration.
    /// * `time_step` - How much to advance the solution.
    /// 
    fn gauss_legendre_integration(mu: f64, b: f64, boundary_conditions: [f64;2], mesh: &Vec<f64>, gauss_step: usize) -> (Array2<f64>,Array2<f64>) {
        
        // First generate the basis
        let linear_basis = LinearBasis::new(mesh).unwrap();
        let basis_len = linear_basis.basis.len();

        // initialize matrix mass_matrix (internal, no boundaries included)
        let mut mass_matrix = ndarray::Array::from_elem((basis_len, basis_len), 0_f64);
        // initialize matrix stiffness_matrix (internal, no boundaries included)
        let mut stiffness_matrix = ndarray::Array::from_elem((basis_len, basis_len), 0_f64);

        for i in 1..(basis_len - 1) {
            // Now we calculate every integral in the equation.
            // One needs to be careful regarding the boundary of the mass_matrix.
            // Obtain every integral. Later on integrals are assigned to the corresponding matrx or vector element.
            let derivative_phi = linear_basis.basis[i].differentiate();
            // replaced by boundary condition for basis[n-1] in vector
            let derivative_phi_next = linear_basis.basis[i+1].differentiate();
            // replaced by boundary condition for basis[0] in vector
            let derivative_phi_prev = linear_basis.basis[i-1].differentiate();

            // Transform intervals from -1,1 to [ai,bi]
            let transform_function_prev = FirstDegreePolynomial::transformation_from_m1_p1(
                mesh[i-1],
                mesh[i],
            );
            let transform_function_next = FirstDegreePolynomial::transformation_from_m1_p1(
                mesh[i],
                mesh[i+1],
            );
            let transform_function_square =
                FirstDegreePolynomial::transformation_from_m1_p1(
                    mesh[i-1],
                    mesh[i+1],
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
                // dot product <phi_j,phi_(j-1)>
                integral_prev_approximation_mass += 
                    linear_basis.basis[i].evaluate(translated_point_prev) *
                    linear_basis.basis[i-1].evaluate(translated_point_prev) * derivative_t_prev.evaluate(x) * w;
                // dot product <phi_j,phi_j>
                integral_square_approximation_mass +=
                    linear_basis.basis[i].evaluate(translated_point_square).powf(2_f64) *
                    derivative_t_square.evaluate(x) * w;
                // dot product <phi_j,phi_(j+1)>
                integral_next_approximation_mass +=
                    linear_basis.basis[i].evaluate(translated_point_next) *
                    linear_basis.basis[i+1].evaluate(translated_point_next) * derivative_t_next.evaluate(x) * w;
                
                // Derivative integrals
                // integral <phi_j',phi_(j-1)'>
                integral_prev_approximation_prime +=
                derivative_phi.evaluate(translated_point_prev) *
                derivative_phi_prev.evaluate(translated_point_prev) * derivative_t_prev.evaluate(x) * w;
                // integral <phi_j',phi_j'>
                integral_square_approximation_prime +=
                derivative_phi.evaluate(translated_point_square).powf(2_f64) *
                derivative_t_square.evaluate(x) * w;
                // integral <phi_j',phi_(j+1)'>
                integral_next_approximation_prime +=
                derivative_phi.evaluate(translated_point_next) *
                derivative_phi_next.evaluate(translated_point_next) * derivative_t_next.evaluate(x) * w;
                
                // Half derivative integrals
                // integral <phi_j,phi_(j-1)'>
                integral_prev_approximation_half += 
                linear_basis.basis[i].evaluate(translated_point_prev) *
                derivative_phi_prev.evaluate(translated_point_prev) * derivative_t_prev.evaluate(x) * w;
                // integral <phi_j,phi_j'>
                integral_square_approximation_half += 
                linear_basis.basis[i].evaluate(translated_point_square) *
                derivative_phi.evaluate(translated_point_square) * derivative_t_square.evaluate(x) * w;
                // integral <phi_j,phi_(j+1)'>
                integral_next_approximation_half += 
                linear_basis.basis[i].evaluate(translated_point_next) *
                derivative_phi_next.evaluate(translated_point_next) * derivative_t_next.evaluate(x) * w;
            }

            if i == 1 {

                // last two approximations to mass mass_matrix are put inside final mass_matrix
                mass_matrix[[i,i]] = integral_square_approximation_mass;
                mass_matrix[[i,i+1]] = integral_next_approximation_mass;

                // stiffness_matrix indices also have the same delay as the mass_matrix

                // left-side boundary condition is added to value of boundary terms.
                // Must be added to u[0] at the end
                mass_matrix[[0,0]] += -integral_prev_approximation_mass * boundary_conditions[0];

            } else if i == basis_len - 2 {

                mass_matrix[[i,basis_len-3]] = integral_prev_approximation_mass;
                mass_matrix[[i,basis_len-2]] = integral_square_approximation_mass;

                //right-side boundary condition is addded to stiffness_matrix
                mass_matrix[[basis_len - 1, basis_len - 1]] += - integral_next_approximation_mass * boundary_conditions[1];

            } else {

                mass_matrix[[i,i-1]] = integral_prev_approximation_mass;
                mass_matrix[[i,i]] = integral_square_approximation_mass;
                mass_matrix[[i,i+1]] = integral_next_approximation_mass;

                
            }

            // add the rest of stiffness_matrix[[i,i-1]] elements
            stiffness_matrix[[i,i-1]] = - mu * integral_prev_approximation_prime -
            b * integral_prev_approximation_half;
            // add the rest of stiffness_matrix[[i,i]] elements
            stiffness_matrix[[i,i]] = - mu * integral_square_approximation_prime -
            b * integral_square_approximation_half;
            // add the rest of stiffness_matrix[[i,i+1]] elements
            stiffness_matrix[[i,i+1]] = - mu * integral_next_approximation_prime -
            b * integral_next_approximation_half;
        }

        mass_matrix[[0,0]] = 1_f64;
        mass_matrix[[basis_len-1,basis_len-1]] = 1_f64;
        stiffness_matrix[[0,0]] = 1_f64;
        stiffness_matrix[[basis_len-1,basis_len-1]] = 1_f64;

        // final result M(u_ti+1) = M(u_ti) + S(delta_t * u_ti)
        // this is the multiplication that has to be done
        // where M is mass matrix, S is stiffness matrix

        (mass_matrix,stiffness_matrix)

    }
}

impl DiffEquationSolver for DiffussionSolverTimeDependent {

    fn solve(&mut self, time_step: f64) -> Result<Vec<f64>, Error> {
        
        println!("Mass Matrix: {:?}\n\n and Stiffness Matrix: {:?}\n\n",self.mass_matrix,self.stiffness_matrix);

        // let b = stiffness_matrix * self.state * time_step + mass_matrix * self.state;
        let b_first_part = utils::tridiagonal_matrix_vector_multiplication(
            &self.stiffness_matrix, &self.state, time_step)?;

        let b_second_part = utils::tridiagonal_matrix_vector_multiplication(
            &self.mass_matrix, &self.state, 1_f64)?;

        let b = utils::add(
            &b_first_part,
            &b_second_part)?;

        let mut res = matrix_solver::solve_by_thomas(&self.mass_matrix, &b)?;

        // reinsert boundary values
        res[0] = self.boundary_conditions[0];
        res[b.len()-1] = self.boundary_conditions[1];
        
        self.state = Array1::from_vec(res.clone());

        Ok(res)

    }
}
#[cfg(test)]
mod tests {

    #[test]
    fn test_matrix_and_vector_values_4p() {

    }
}
