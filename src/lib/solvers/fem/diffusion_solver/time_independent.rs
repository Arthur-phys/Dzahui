use crate::solvers::fem::basis::single_variable::{
    linear_basis::LinearBasis, polynomials_1d::FirstDegreePolynomial, Differentiable1D, Function1D,
};
use crate::solvers::matrix_solver;
use crate::solvers::quadrature::gauss_legendre;
use crate::solvers::DiffEquationSolver;
use crate::Error;

use ndarray::{Array1, Array2};

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
pub struct DiffussionSolverTimeIndependent {
    pub boundary_conditions: [f64; 2],
    pub(crate) stiffness_matrix: Array2<f64>,
    pub(crate) b_vector: Array1<f64>,
    pub gauss_step: usize,
    pub mu: f64,
    pub b: f64,
}

impl DiffussionSolverTimeIndependent {
    /// Creates new instance
    pub fn new(boundary_conditions: [f64; 2], mesh: Vec<f64>, mu: f64, b: f64, gauss_step: usize) -> Self {

        let (stiffness_matrix, b_vector) = Self::gauss_legendre_integration(boundary_conditions, mu, b, &mesh, gauss_step);

        Self {
            boundary_conditions,
            stiffness_matrix,
            gauss_step,
            b_vector,
            mu,
            b,
        }
    }

    /// # General Information
    ///
    /// First, it generates the basis for a solver from the linear basis constructor.
    /// Then the stiffnes matrix and vector b are generated based on linear basis integration via Gauss-Legendre.
    /// The matrix and vector b generated are only for internal nodes of a given mesh because boundary nodes are fixed and given as boundary conditions.
    /// The previous statement means that both of them are alwas `basis.len() - 2` long on their respective dimensions.
    /// Basis length 3 or 4 cases are treated differently, since the only integral to be made are the ones that include first and last basis elements.
    ///
    /// # Parameters
    ///
    /// * `&self` - An instance of `DiffussionSolverTimeIndependent`.
    /// * `gauss_step` - how many nodes will be calculated for a given integration.
    ///
    /// # Returns
    ///
    /// A tuple with both the stiffness matrix and the vector b.
    ///
    pub fn gauss_legendre_integration(boundary_conditions: [f64;2], mu: f64, b: f64, mesh: &Vec<f64>, gauss_step: usize) -> (Array2<f64>, Array1<f64>) {
        
        let basis = LinearBasis::new(mesh).unwrap();
        let basis_len = basis.basis.len();

        let mut stiffness_matrix =
            ndarray::Array::from_elem((basis_len, basis_len), 0_f64);
        
        let mut b_vector = Array1::from_elem(basis_len, 0_f64);


        for i in 1..(basis_len - 1) {

            let derivative_phi = basis.basis[i].differentiate();

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

            let derivative_t_prev = transform_function_prev.differentiate();
            let derivative_t_next = transform_function_next.differentiate();
            let derivative_t_square = transform_function_square.differentiate();

            let derivative_prev = basis.basis[i - 1].differentiate();
            let derivative_next = basis.basis[i + 1].differentiate();

            let mut integral_prev_approximation = 0_f64;
            let mut integral_next_approximation = 0_f64;
            let mut integral_square_approximation = 0_f64;

            // integrate
            for j in 1..gauss_step {
                // Obtaining arccos(node) and weight
                let (theta, w) = gauss_legendre::quad_pair(gauss_step, j);
                let x = theta.cos();

                // translated from -1,1
                let translated_point_prev = transform_function_prev.evaluate(x);
                let translated_point_next = transform_function_next.evaluate(x);
                let translated_point_square = transform_function_square.evaluate(x);

                integral_prev_approximation += (mu
                    * derivative_phi.evaluate(translated_point_prev)
                    * derivative_prev.evaluate(translated_point_prev)
                    + b
                        * derivative_prev.evaluate(translated_point_prev)
                        * basis.basis[i].evaluate(translated_point_prev))
                    * derivative_t_prev.evaluate(x)
                    * w;
                integral_next_approximation += (mu
                    * derivative_phi.evaluate(translated_point_next)
                    * derivative_next.evaluate(translated_point_next)
                    + b
                        * derivative_next.evaluate(translated_point_next)
                        * basis.basis[i].evaluate(translated_point_next))
                    * derivative_t_next.evaluate(x)
                    * w;
                integral_square_approximation += (mu
                    * derivative_phi.evaluate(translated_point_square)
                    * derivative_phi.evaluate(translated_point_square)
                    + b
                        * derivative_phi.evaluate(translated_point_square)
                        * basis.basis[i].evaluate(translated_point_square))
                    * derivative_t_square.evaluate(x)
                    * w;
            }

            stiffness_matrix[[i, i]] = integral_square_approximation;
            stiffness_matrix[[i, i - 1]] = integral_prev_approximation;
            stiffness_matrix[[i, i + 1]] = integral_next_approximation;
        
        }

        // adjusting boundary conditions inside vector and matrix so that u_0 = boundary_conditions[left] and u[n] = boundary_codnitions[right]
        // when multiplying
        stiffness_matrix[[0,0]] = 1_f64;
        stiffness_matrix[[basis_len - 1, basis_len - 1]] = 1_f64;
        b_vector[0] = boundary_conditions[0];
        b_vector[basis_len - 1] = boundary_conditions[1];

        (stiffness_matrix, b_vector)
    }
}

impl DiffEquationSolver for DiffussionSolverTimeIndependent {
    /// # Specific implementation
    ///
    /// Solving starts by obtaining stiffness matrix and vector b (Ax=b).
    /// Then both are used inside function `solve_by_thomas` to obtain the result vector.
    /// Result vector has 2 extra entries: one at the beggining and one at the end. They correspond to boundary value conditions, which are set at the very
    /// end since they do not change.
    ///
    fn solve(&mut self, _time_step: f64) -> Result<Vec<f64>, Error> {

        let res = matrix_solver::solve_by_thomas(&self.stiffness_matrix, &self.b_vector)?;

        Ok(res)
    }
}

#[cfg(test)]
mod test {

    use crate::solvers::matrix_solver;

    use super::DiffussionSolverTimeIndependent;

    #[test]
    fn regular_mesh_matrix_3p() {
        let dif_solver = DiffussionSolverTimeIndependent::new(
            [0_f64, 1_f64],
            vec![0_f64, 0.5, 1_f64],
            1_f64,
            1_f64,
            150
        );

        assert!(dif_solver.stiffness_matrix[[0,0]] == 1_f64);
        assert!(dif_solver.stiffness_matrix[[1, 1]] <= 4.1 && dif_solver.stiffness_matrix[[1, 1]] >= 3.9);
        assert!(dif_solver.stiffness_matrix[[1, 2]] <= -1.4 && dif_solver.stiffness_matrix[[1, 1]] >= -1.6);
        assert!(dif_solver.stiffness_matrix[[1, 0]] <= -2.4 && dif_solver.stiffness_matrix[[1, 1]] >= -2.6);
        assert!(dif_solver.stiffness_matrix[[2,2]] == 1_f64);
    }

    #[test]
    fn solve_system_3p() {
        let dif_solver = DiffussionSolverTimeIndependent::new(
            [0_f64, 1_f64],
            vec![0_f64, 0.5, 1_f64],
            1_f64,
            1_f64,
            150,
        );

        let res = matrix_solver::solve_by_thomas(&dif_solver.stiffness_matrix, &dif_solver.b_vector).unwrap();

        assert!(res.len() == 3);
        assert!(res[1] >= 0.2 && res[1] <= 0.4);
    }

    #[test]
    fn regular_mesh_matrix_4p() {
        let dif_solver = DiffussionSolverTimeIndependent::new(
            [0_f64, 1_f64],
            vec![0_f64, 0.33, 0.66, 1_f64],
            1_f64,
            1_f64,
            150
        );

        assert!(dif_solver.stiffness_matrix[[1, 0]] <= -3.4 && dif_solver.stiffness_matrix[[1, 2]] >= -3.6);
        assert!(dif_solver.stiffness_matrix[[1, 1]] <= 6.1 && dif_solver.stiffness_matrix[[1, 1]] >= 5.9);
        assert!(dif_solver.stiffness_matrix[[1, 2]] <= -2.4 && dif_solver.stiffness_matrix[[1, 2]] >= -2.6);
        assert!(dif_solver.stiffness_matrix[[2, 1]] <= -3.4 && dif_solver.stiffness_matrix[[2, 1]] >= -3.6);
        assert!(dif_solver.stiffness_matrix[[2, 2]] <= 6.1 && dif_solver.stiffness_matrix[[2, 2]] >= 5.9);
        assert!(dif_solver.stiffness_matrix[[2, 3]] <= -2.3 && dif_solver.stiffness_matrix[[2, 2]] >= -2.5);
        assert!(dif_solver.stiffness_matrix[[0, 0]] == 1_f64);
        assert!(dif_solver.stiffness_matrix[[3, 3]] == 1_f64);

    }

    #[test]
    fn solve_system_4p() {
        let dif_solver = DiffussionSolverTimeIndependent::new(
            [0_f64, 1_f64],
            vec![0_f64, 0.33, 0.66, 1_f64],
            1_f64,
            1_f64,
            150
        );

        let res = matrix_solver::solve_by_thomas(&dif_solver.stiffness_matrix, &dif_solver.b_vector).unwrap();

        assert!(res.len() == 4);
        assert!(res[0] == 0_f64);
        assert!(res[1] >= 0.20 && res[1] <= 0.24);
        assert!(res[2] >= 0.52 && res[2] <= 0.56);
        assert!(res[3] == 1_f64);
    }

    #[test]
    fn regular_mesh_bigger_matrix() {
        let dif_solver = DiffussionSolverTimeIndependent::new(
            [0_f64, 1_f64],
            vec![0_f64, 0.25, 0.5, 0.75, 1_f64],
            1_f64,
            1_f64,
            150
        );

        assert!(dif_solver.stiffness_matrix[[1, 0]] <= -4.4 && dif_solver.stiffness_matrix[[1, 2]] >= -4.6);
        assert!(dif_solver.stiffness_matrix[[1, 1]] <= 8.1 && dif_solver.stiffness_matrix[[1, 1]] >= 7.9);
        assert!(dif_solver.stiffness_matrix[[1, 2]] <= -3.4 && dif_solver.stiffness_matrix[[1, 2]] >= -3.6);
        assert!(dif_solver.stiffness_matrix[[2, 1]] <= -4.4 && dif_solver.stiffness_matrix[[2, 1]] >= -4.6);
        assert!(dif_solver.stiffness_matrix[[2, 2]] <= 8.1 && dif_solver.stiffness_matrix[[2, 2]] >= 7.9);
        assert!(dif_solver.stiffness_matrix[[2, 3]] <= -3.4 && dif_solver.stiffness_matrix[[2, 3]] >= -3.6);
        assert!(dif_solver.stiffness_matrix[[3, 2]] <= -4.4 && dif_solver.stiffness_matrix[[3, 2]] >= -4.6);
        assert!(dif_solver.stiffness_matrix[[3, 3]] <= 8.1 && dif_solver.stiffness_matrix[[3, 3]] >= 7.9);
        assert!(dif_solver.stiffness_matrix[[3, 4]] <= -3.4 && dif_solver.stiffness_matrix[[3, 3]] >= -3.6);
        assert!(dif_solver.stiffness_matrix[[4, 4]] == 1_f64);
        assert!(dif_solver.stiffness_matrix[[0, 0]] == 1_f64);

        assert!(dif_solver.b_vector[0] == dif_solver.boundary_conditions[0]);
        assert!(dif_solver.b_vector[4] == dif_solver.boundary_conditions[1]);
    }

    #[test]
    fn solve_bigger_system() {
        let dif_solver = DiffussionSolverTimeIndependent::new(
            [0_f64, 1_f64],
            vec![0_f64, 0.25, 0.5, 0.75, 1_f64],
            1_f64,
            1_f64,
            150
        );

        let res = matrix_solver::solve_by_thomas(&dif_solver.stiffness_matrix, &dif_solver.b_vector).unwrap();

        assert!(res.len() == 5);
        assert!(res[0] == dif_solver.boundary_conditions[0]);
        assert!(res[1] >= 0.15 && res[1] <= 0.17);
        assert!(res[2] >= 0.36 && res[2] <= 0.38);
        assert!(res[3] >= 0.63 && res[3] <= 0.655);
        assert!(res[4] == dif_solver.boundary_conditions[1]);
    }
}
