// internal dependencies
use crate::solvers::fem::basis::single_variable::{
    linear_basis::LinearBasis, polynomials_1d::FirstDegreePolynomial, Differentiable1D, Function1D,
};
use crate::solvers::{solver_trait::DiffEquationSolver, matrix_solver, utils, quadrature::gauss_legendre};
use crate::Error;

// External dependencies
use ndarray::{Array1, Array2};


#[derive(Debug)]
///
/// # General Conditions
/// 
/// An enum representing wether initial conditions are initialized or not.
/// Can be set to default.
/// 
/// # Arms
/// 
/// * `Uninitialized` - Conditions are not present
/// * `Are` - Conditions are present
/// 
pub(crate) enum Conditions {
    Uninitialized,
    Are(Vec<f64>)
}

impl Default for Conditions {
    fn default() -> Self {
        Conditions::Uninitialized
    }
}

#[derive(Default,Debug)]
///
/// # General Information
/// 
/// A struct representing every param to solve the time-dependent equation including intial conditions.
/// 
/// # Params
/// 
/// * `mu` - Movement term
/// * `b` - Velocity term
/// * `boundary_conditions` - Dirichlet conditions
/// * `initial_conditions` - Internal initial conditions
/// 
pub struct DiffussionParamsTimeDependent {
    pub mu: f64,
    pub b: f64,
    pub boundary_conditions: [f64;2],
    pub(crate) initial_conditions: Conditions
}

impl DiffussionParamsTimeDependent {
    /// Set mu
    pub fn mu(self, mu: f64) -> Self {
        Self {
            mu,
            ..self
        }
    }

    /// Set b
    pub fn b(self, b: f64) -> Self {
        Self {
            b,
            ..self
        }
    }

    /// Set boundary conditions
    pub fn boundary_conditions(self, left: f64, right: f64) -> Self {
        Self {
            boundary_conditions: [left, right],
            ..self
        }
    }

    /// Set initial conditions from a vector
    pub fn initial_conditions<A: IntoIterator<Item = f64>>(self, initial_conditions: A) -> Self {
        Self {
            initial_conditions: Conditions::Are(initial_conditions.into_iter().collect()),
            ..self
        }
    }
}
#[derive(Debug)]
/// # General Information
///
/// A diffusion solver with time-dependence abstracts the equation: "u_t - μu_xx + bu_x = 0" and contains boundary conditions,
/// initial conditions, mesh, "stiffness_matrix" and "μ".
///
/// # Fields
///
/// * `boundary_conditions` - Boundary conditions (Only dirichlet is supported for now, Neumann is being worked on)
/// * `stiffness_matrix` - Matrix of elements that is multiplied by time
/// * `initial_conditions` - Every internal point needs an initial condition to advance the solution in time
/// * `mass_matrix` - A matrix that pertains only to elements that are not multiplied by time
/// * `integration_step` - Amount of terms to sum over to make the integral
/// * `state` - The state of every point at time t
/// * `mu` - First ot two needed constants
/// * `b` - Second of two needed constants
///
pub struct DiffussionSolverTimeDependent {
    pub boundary_conditions: [f64; 2],
    pub(crate) stiffness_matrix: Array2<f64>,
    pub initial_conditions: Vec<f64>,
    pub(crate) mass_matrix: Array2<f64>,
    pub integration_step: usize,
    pub(crate) state: Array1<f64>,
    pub mu: f64,
    pub b: f64,
}

impl DiffussionSolverTimeDependent {
    /// Creates new instance checking initial conditions are the size they should be.
    pub fn new(params: &DiffussionParamsTimeDependent, mesh: Vec<f64>, integration_step: usize) -> Result<Self,Error> {
        
        let initial_conditions = match &params.initial_conditions {
            Conditions::Uninitialized => {
                vec![0_f64;mesh.len()]
            },
            Conditions::Are(vec) => {
                vec.clone()
            }
        };
        
        if initial_conditions.len() != mesh.len() - 2 {
            return Err(Error::WrongDims)
        }

        // obtain general initial state
        let mut state = vec![0_f64;mesh.len()];
        state[0] = params.boundary_conditions[0];
        state[mesh.len() - 1] = params.boundary_conditions[1]; 
        for i in 1..(mesh.len() - 1) {
            state[i] = initial_conditions[i-1];
        }

        let state = Array1::from_vec(state);

        let (mass_matrix, stiffness_matrix) = Self::gauss_legendre_integration(
                params.mu, params.b, &mesh, integration_step)?;

        // obtain matrices

        Ok(Self {
            boundary_conditions: params.boundary_conditions,
            initial_conditions,
            stiffness_matrix,
            integration_step,
            mass_matrix,
            state,
            mu: params.mu,
            b: params.b,
        })
    }

    /// # General Information
    /// 
    /// Compĺete integration of linear basis to obtain mass matrix and stiffness matrix.
    /// Corners of every element have special values to attone for boundary conditions being constant.
    /// Matrices serve to solve the resulting problem: M(u_ti+1) = M(u_ti) + S(delta_t * u_ti) where M is mass matrix and S is stiffness matrix.
    /// 
    /// # Parameters
    /// 
    /// * `mu` - First of two terms to solve equation
    /// * `b` - Second of two terms to solve equation
    /// * `mesh` - Vector of f64 representing a mesh
    /// * `gauss_step` - Amount of nodes to compute for integration.
    /// 
    fn gauss_legendre_integration(mu: f64, b: f64, mesh: &Vec<f64>, gauss_step: usize) -> Result<(Array2<f64>,Array2<f64>),Error> {
        
        // First generate the basis
        let linear_basis = LinearBasis::new(mesh)?;
        let basis_len = linear_basis.basis.len();

        // initialize matrix mass_matrix (boundaries included)
        let mut mass_matrix = ndarray::Array::from_elem((basis_len, basis_len), 0_f64);
        // initialize matrix stiffness_matrix (boundaries included)
        let mut stiffness_matrix = ndarray::Array::from_elem((basis_len, basis_len), 0_f64);

        for i in 1..(basis_len - 1) {
            // Now we calculate every integral in the equation.
            // One needs to be careful regarding the boundary of the mass_matrix.
            // Obtain every integral. Later on integrals are assigned to the corresponding matrx or vector element.
            let derivative_phi = linear_basis.basis[i].differentiate()?;
            // replaced by boundary condition for basis[n-1] in vector
            let derivative_phi_next = linear_basis.basis[i+1].differentiate()?;
            // replaced by boundary condition for basis[0] in vector
            let derivative_phi_prev = linear_basis.basis[i-1].differentiate()?;

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
            let derivative_t_prev = transform_function_prev.differentiate()?;
            let derivative_t_next = transform_function_next.differentiate()?;
            let derivative_t_square = transform_function_square.differentiate()?;
            
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
                let (theta, w) = gauss_legendre::quad_pair(gauss_step, j)?;
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

            mass_matrix[[i,i-1]] = integral_prev_approximation_mass;
            mass_matrix[[i,i]] = integral_square_approximation_mass;
            mass_matrix[[i,i+1]] = integral_next_approximation_mass;

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

        Ok((mass_matrix,stiffness_matrix))

    }
}

impl DiffEquationSolver for DiffussionSolverTimeDependent {

    /// # Specific implementation
    /// 
    /// Calculate a vector b on left-side of equation.
    /// Then solve problem Ax = b for x.
    /// 
    fn solve(&mut self, time_step: f64) -> Result<Vec<f64>, Error> {

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
    use crate::solvers::{solver_trait::DiffEquationSolver, diffusion_solver::DiffussionParams};

    use super::DiffussionSolverTimeDependent;


    #[test]
    fn test_matrix_and_vector_values_3p() {

        let conditions = DiffussionParams::time_dependent()
            .b(1_f64)
            .mu(1_f64)
            .boundary_conditions(0_f64, 1_f64)
            .initial_conditions(vec![0_f64;1]);

        let dif_solver = DiffussionSolverTimeDependent::new(
            &conditions,
            vec![0_f64,0.5,1_f64],
            150)
            .unwrap();

            println!("{:?}",dif_solver.stiffness_matrix);

            assert!(dif_solver.mass_matrix[[0,0]] == 1_f64);
            assert!(dif_solver.mass_matrix[[1,0]] >= 0.08 && dif_solver.mass_matrix[[1,0]] <= 0.09);
            assert!(dif_solver.mass_matrix[[1,1]] >= 0.3 && dif_solver.mass_matrix[[1,1]] <= 0.35);
            assert!(dif_solver.mass_matrix[[1,2]] >= 0.08 && dif_solver.mass_matrix[[1,2]] <= 0.09);
            assert!(dif_solver.mass_matrix[[2,2]] == 1_f64);

            assert!(dif_solver.stiffness_matrix[[0,0]] == 1_f64);
            assert!(dif_solver.stiffness_matrix[[1,0]] >= 2.4 && dif_solver.stiffness_matrix[[1,0]] <= 2.6);
            assert!(dif_solver.stiffness_matrix[[1,1]] >= -4.1 && dif_solver.stiffness_matrix[[1,1]] <= -3.9);
            assert!(dif_solver.stiffness_matrix[[1,2]] >= 1.4 && dif_solver.stiffness_matrix[[1,2]] <= 1.6);
            assert!(dif_solver.stiffness_matrix[[2,2]] == 1_f64);
    }

    #[test]
    fn test_matrix_solved_3p() {

        let conditions = DiffussionParams::time_dependent()
            .b(1_f64)
            .mu(1_f64)
            .boundary_conditions(1_f64, 0_f64)
            .initial_conditions(vec![15_f64;1]);

        let mut dif_solver = DiffussionSolverTimeDependent::new(
            &conditions,
            vec![0_f64,0.5,1_f64],
            150)
            .unwrap();

        for _i in 0..1000 {
            dif_solver.solve(0.01).unwrap();
        }

        assert!(dif_solver.state[0] == 1_f64);
        assert!(dif_solver.state[2] == 0_f64);
        assert!(dif_solver.state[1] <= 0.65 && dif_solver.state[1] >= 0.55);


    }
}
