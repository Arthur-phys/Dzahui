use crate::solvers::fem::basis::single_variable::{linear_basis::LinearBasis, polynomials_1d::FirstDegreePolynomial, Function1D, Differentiable1D};
use crate::solvers::DiffEquationSolver;
use crate::solvers::{linear_solver::ThomasSolver, quadrature::GaussLegendreQuadrature};
use crate::Error;

use ndarray::{Array, Array1, Ix1, Ix2};

#[derive(Debug)]
/// # General Information
///
/// A diffusion solver abstracts the equation to solve: "μu'' + bu' = 0" and contains boundary conditions along with mesh, "b" and "μ"
///
/// # Fields
///
/// * `boundary_conditions` - Original boundary conditions (Only dirichlet is supported for now, Neumann is being worked on).
/// * `mesh` - A vector of floats representing a line.
/// * `mu` - First ot two needed constants.
/// * `b` - Second of two needed constants.
///
pub struct DiffussionSolver {
    boundary_conditions: [f64; 2],
    mesh: Vec<f64>,
    mu: f64,
    b: f64,
}

impl DiffussionSolver {
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

impl ThomasSolver for DiffussionSolver {}

impl DiffEquationSolver for DiffussionSolver {

    fn solve(&self) -> Result<Array1<f64>, Error> {
        let (a, b) = self.gauss_legendre_integration(150);

        let mut res = Self::solve_by_thomas(&a, &b)?;

        // res[1] += b[0];
        // res[b.len()] += b[b.len() - 1];

        // Adding boundary condition values
        res[0] = self.boundary_conditions[0];
        res[b.len() + 1] = self.boundary_conditions[1];

        Ok(res)
    }

}

impl GaussLegendreQuadrature for DiffussionSolver {
    
    fn gauss_legendre_integration(&self, gauss_step: usize) -> (Array<f64, Ix2>, Array<f64, Ix1>) {

        let basis = LinearBasis::new(&self.mesh).unwrap();
        let long_basis = basis.basis.len();

        let mut stiffness_matrix =
            ndarray::Array::from_elem((long_basis - 2, long_basis - 2), 0_f64);

        if long_basis - 2 == 1 {

            let derivative_phi = basis.basis[1].differentiate();
            let transform_function = FirstDegreePolynomial::transformation_from_m1_p1(self.mesh[0], self.mesh[2]);
            let derivative_t = transform_function.differentiate();
            // initial value
            let mut integral_square_approximation = 0_f64;

            for j in 1..gauss_step {
                // Obtaining arccos(node) and weight
                let (theta, w) = Self::quad_pair(gauss_step, j);
                let x = theta.cos();

                // translated to -1,1
                let translated_point_square = transform_function.evaluate(x);

                integral_square_approximation += (self.mu
                    * derivative_phi.evaluate(translated_point_square)
                    * derivative_phi.evaluate(translated_point_square)
                    + self.b
                        * derivative_phi.evaluate(translated_point_square)
                        * basis.basis[1].evaluate(translated_point_square))
                    * derivative_t.evaluate(x)
                    * w;
            }

            stiffness_matrix[[0, 0]] = integral_square_approximation;

        } else {
            if long_basis - 2 == 2 {
            } else {
                for i in 2..long_basis - 2 {
                    let derivative_phi = basis.basis[i].differentiate();

                    let transform_function_prev =
                        transformation_factory.build_to_m1_p1(self.mesh[i - 1], self.mesh[i]);
                    let transform_function_next =
                        transformation_factory.build_to_m1_p1(self.mesh[i], self.mesh[i + 1]);
                    let transform_function_square =
                        transformation_factory.build_to_m1_p1(self.mesh[i - 1], self.mesh[i + 1]);
                    let derivative_t_prev = transform_function_prev.differentiate();
                    let derivative_t_next = transform_function_next.differentiate();
                    let derivative_t_square = transform_function_square.differentiate();

                    let derivative_prev = basis.basis[i - 1].differentiate();
                    let derivative_next = basis.basis[i + 1].differentiate();

                    let mut integral_prev_approximation = 0_f64;
                    let mut integral_next_approximation = 0_f64;
                    let mut integral_square_approximation = 0_f64;

                    for j in 1..gauss_step {
                        // Obtaining arccos(node) and weight
                        let (theta, w) = Self::quad_pair(gauss_step, j);
                        let x = theta.cos();

                        // translated to -1,1
                        let translated_point_prev = transform_function_prev.evaluate(x);
                        let translated_point_next = transform_function_next.evaluate(x);
                        let translated_point_square = transform_function_square.evaluate(x);

                        integral_prev_approximation += (self.mu
                            * derivative_phi.evaluate(translated_point_prev)
                            * derivative_prev.evaluate(translated_point_prev)
                            + self.b
                                * derivative_phi.evaluate(translated_point_prev)
                                * basis.basis[i - 1].evaluate(translated_point_prev))
                            * derivative_t_prev.evaluate(x)
                            * w;
                        integral_next_approximation += (self.mu
                            * derivative_phi.evaluate(translated_point_next)
                            * derivative_next.evaluate(translated_point_next)
                            + self.b
                                * derivative_phi.evaluate(translated_point_next)
                                * basis.basis[i + 1].evaluate(translated_point_next))
                            * derivative_t_next.evaluate(x)
                            * w;
                        integral_square_approximation += (self.mu
                            * derivative_phi.evaluate(translated_point_square)
                            * derivative_phi.evaluate(translated_point_square)
                            + self.b
                                * derivative_phi.evaluate(translated_point_square)
                                * basis.basis[i].evaluate(translated_point_square))
                            * derivative_t_square.evaluate(x)
                            * w;
                    }
                    stiffness_matrix[[i - 1, i - 2]] = integral_next_approximation;
                    stiffness_matrix[[i - 1, i]] = integral_prev_approximation;
                    stiffness_matrix[[i - 1, i - 1]] = integral_square_approximation;
                }
            }

            // elements here are special cases which only present a single bilinear evaluation either on the right or on the left
            let derivative_phi_zero_internal = basis.basis[1].differentiate();
            let derivative_phi_last_internal = basis.basis[long_basis - 2].differentiate();

            let transform_function_zero_internal =
                transformation_factory.build_to_m1_p1(self.mesh[1], self.mesh[2]);
            let transform_function_zero_square =
                transformation_factory.build_to_m1_p1(self.mesh[0], self.mesh[2]);
            let transform_function_last_internal = transformation_factory
                .build_to_m1_p1(self.mesh[long_basis - 3], self.mesh[long_basis - 2]);
            let transform_function_last_square = transformation_factory
                .build_to_m1_p1(self.mesh[long_basis - 3], self.mesh[long_basis - 1]);

            let derivative_t_zero = transform_function_zero_internal.differentiate();
            let derivative_t_last = transform_function_last_internal.differentiate();
            let derivative_t_zq = transform_function_zero_square.differentiate();
            let derivative_t_lq = transform_function_last_square.differentiate();

            let derivative_one_internal = basis.basis[2].differentiate();
            let derivative_pen_internal = basis.basis[long_basis - 3].differentiate();

            let mut integral_zero_internal_square_approximation = 0_f64;
            let mut integral_zero_internal_one_approximation = 0_f64;
            let mut integral_last_internal_square_approximation = 0_f64;
            let mut integral_last_internal_pen_approximation = 0_f64;

            for i in 1..gauss_step {
                // Obtaining arccos(node) and weight
                let (theta, w) = Self::quad_pair(gauss_step, i);
                let x = theta.cos();

                // translated to original interval
                let translated_point_zero = transform_function_zero_internal.evaluate(x);
                let translated_point_zs = transform_function_zero_square.evaluate(x);
                let translated_point_last = transform_function_last_internal.evaluate(x);
                let translated_point_ls = transform_function_last_square.evaluate(x);

                integral_zero_internal_square_approximation += (self.mu
                    * derivative_phi_zero_internal.evaluate(translated_point_zs)
                    * derivative_phi_zero_internal.evaluate(translated_point_zs)
                    + self.b
                        * derivative_phi_zero_internal.evaluate(translated_point_zs)
                        * basis.basis[1].evaluate(translated_point_zs))
                    * derivative_t_zq.evaluate(x)
                    * w;
                integral_zero_internal_one_approximation += (self.mu
                    * derivative_phi_zero_internal.evaluate(translated_point_zero)
                    * derivative_one_internal.evaluate(translated_point_zero)
                    + self.b
                        * derivative_one_internal.evaluate(translated_point_zero)
                        * basis.basis[1].evaluate(translated_point_zero))
                    * derivative_t_zero.evaluate(x)
                    * w;
                integral_last_internal_square_approximation += (self.mu
                    * derivative_phi_last_internal.evaluate(translated_point_ls)
                    * derivative_phi_last_internal.evaluate(translated_point_ls)
                    + self.b
                        * derivative_phi_last_internal.evaluate(translated_point_ls)
                        * basis.basis[long_basis - 2].evaluate(translated_point_ls))
                    * derivative_t_lq.evaluate(x)
                    * w;
                integral_last_internal_pen_approximation += (self.mu
                    * derivative_phi_last_internal.evaluate(translated_point_last)
                    * derivative_pen_internal.evaluate(translated_point_last)
                    + self.b
                        * derivative_pen_internal.evaluate(translated_point_last)
                        * basis.basis[long_basis - 2].evaluate(translated_point_last))
                    * derivative_t_last.evaluate(x)
                    * w;
            }

            stiffness_matrix[[0, 0]] = integral_zero_internal_square_approximation;
            stiffness_matrix[[0, 1]] = integral_zero_internal_one_approximation;
            stiffness_matrix[[long_basis - 3, long_basis - 3]] =
                integral_last_internal_square_approximation;
            stiffness_matrix[[long_basis - 3, long_basis - 4]] =
                integral_last_internal_pen_approximation;
        }

        // elements here only serve to impose boundary conditions
        let derivative_phi_zero = basis.basis[0].differentiate();
        let derivative_phi_last = basis.basis[long_basis - 1].differentiate();

        let transform_function_zero =
            transformation_factory.build_to_m1_p1(self.mesh[0], self.mesh[1]);
        let transform_function_last = transformation_factory
            .build_to_m1_p1(self.mesh[long_basis - 2], self.mesh[long_basis - 1]);

        let derivative_t_zero = transform_function_zero.differentiate();
        let derivative_t_last = transform_function_last.differentiate();

        let derivative_one_internal = basis.basis[1].differentiate();
        let derivative_pen_internal = basis.basis[long_basis - 2].differentiate();

        let mut integral_zero_one_approximation = 0_f64;
        let mut integral_last_pen_approximation = 0_f64;

        for i in 1..gauss_step {
            // Obtaining arccos(node) and weight
            let (theta, w) = Self::quad_pair(gauss_step, i);
            let x = theta.cos();

            // translated to original interval
            let translated_point_zero = transform_function_zero.evaluate(x);
            let translated_point_last = transform_function_last.evaluate(x);

            integral_zero_one_approximation += (self.mu
                * derivative_phi_zero.evaluate(translated_point_zero)
                * derivative_one_internal.evaluate(translated_point_zero)
                + self.b
                    * derivative_phi_zero.evaluate(translated_point_zero)
                    * basis.basis[1].evaluate(translated_point_zero))
                * derivative_t_zero.evaluate(x)
                * w;
            integral_last_pen_approximation += (self.mu
                * derivative_phi_last.evaluate(translated_point_last)
                * derivative_pen_internal.evaluate(translated_point_last)
                + self.b
                    * derivative_phi_last.evaluate(translated_point_last)
                    * basis.basis[long_basis - 2].evaluate(translated_point_last))
                * derivative_t_last.evaluate(x)
                * w;
        }

        let mut b_vector = Array1::from_elem(long_basis - 2, 0_f64);
        b_vector[[0]] += -integral_zero_one_approximation * self.boundary_conditions[0];
        b_vector[[long_basis - 3]] += -integral_last_pen_approximation * self.boundary_conditions[1];

        (stiffness_matrix, b_vector)
    }
}

#[cfg(test)]
mod test {

    use crate::solvers::{
        linear_solver::ThomasSolver, quadrature::GaussLegendreQuadrature,
    };

    use super::DiffussionSolver;

    #[test]
    fn regular_mesh_matrix_3p() {
        let dif_solver =
            DiffussionSolver::new([0_f64, 1_f64], vec![0_f64, 0.5, 1_f64], 1_f64, 1_f64);
        let (a, b) = dif_solver.gauss_legendre_integration(150);

        assert!(a[[0, 0]] <= 4.1 && a[[0, 0]] >= 3.9);
        assert!(b[0] <= 1.6 && b[0] >= 1.4);
    }

    #[test]
    fn solve_system_3p() {
        let dif_solver =
            DiffussionSolver::new([0_f64, 1_f64], vec![0_f64, 0.5, 1_f64], 1_f64, 1_f64);

        let (a, b) = dif_solver.gauss_legendre_integration(150);

        let res = DiffussionSolver::solve_by_thomas(&a, &b).unwrap();

        assert!(res.len() == 3);
        assert!(res[1] >= 0.2 && res[1] <= 0.4);
    }

    #[test]
    fn regular_mesh_matrix_4p() {
        let dif_solver =
            DiffussionSolver::new([0_f64, 1_f64], vec![0_f64, 0.33, 0.66, 1_f64], 1_f64, 1_f64);
        let (a, b) = dif_solver.gauss_legendre_integration(150);

        assert!(a[[0, 0]] <= 6.1 && a[[0, 0]] >= 5.9);
        assert!(a[[1, 0]] <= -3.4 && a[[1, 0]] >= -3.6);
        assert!(a[[0, 1]] <= -2.4 && a[[0, 1]] >= -2.6);
        assert!(a[[1, 1]] <= 6.1 && a[[1, 1]] >= 5.9);

        assert!(b[0] == 0_f64);
        assert!(b[1] >= 2.4 && b[1] <= 2.6);
    }

    #[test]
    fn solve_system_4p() {
        let dif_solver =
            DiffussionSolver::new([0_f64, 1_f64], vec![0_f64, 0.33, 0.66, 1_f64], 1_f64, 1_f64);
        let (a, b) = dif_solver.gauss_legendre_integration(150);

        let res = DiffussionSolver::solve_by_thomas(&a, &b).unwrap();

        assert!(res.len() == 4);
        assert!(res[1] >= 0.20 && res[1] <= 0.24);
        assert!(res[2] >= 0.52 && res[2] <= 0.56);
    }

    #[test]
    fn regular_mesh_bigger_matrix() {
        let dif_solver = DiffussionSolver::new(
            [0_f64, 1_f64],
            vec![0_f64, 0.25, 0.5, 0.75, 1_f64],
            1_f64,
            1_f64,
        );
        let (a, b) = dif_solver.gauss_legendre_integration(150);

        assert!(a[[0, 0]] <= 8.1 && a[[0, 0]] >= 7.9);
        assert!(a[[0, 1]] <= -3.4 && a[[0, 1]] >= -3.6);
        assert!(a[[1, 0]] <= -4.4 && a[[1, 0]] >= -4.6);
        assert!(a[[1, 1]] <= 8.1 && a[[1, 1]] >= 7.9);
        assert!(a[[1, 2]] <= -3.4 && a[[1, 2]] >= -3.6);
        assert!(a[[2, 1]] <= -4.4 && a[[2, 1]] >= -4.6);
        assert!(a[[2, 2]] <= 8.1 && a[[2, 2]] >= 7.9);

        assert!(b[b.len() - 1] >= 3.4 && b[b.len() - 1] <= 3.6);
    }

    #[test]
    fn solve_bigger_system() {
        let dif_solver = DiffussionSolver::new(
            [0_f64, 1_f64],
            vec![0_f64, 0.25, 0.5, 0.75, 1_f64],
            1_f64,
            1_f64,
        );
        let (a, b) = dif_solver.gauss_legendre_integration(150);

        let res = DiffussionSolver::solve_by_thomas(&a, &b).unwrap();

        assert!(res.len() == 5);
        assert!(res[1] >= 0.15 && res[1] <= 0.17);
        assert!(res[2] >= 0.36 && res[2] <= 0.38);
        assert!(res[3] >= 0.63 && res[3] <= 0.655);
    }

    #[test]
    fn obtain_non_homogeneous_solution() {
        let dif_solver = DiffussionSolver::new(
            [0_f64, 1_f64],
            vec![0_f64, 0.25, 0.5, 0.75, 1_f64],
            1_f64,
            1_f64,
        );

        let (a,b) = dif_solver.gauss_legendre_integration(150);
        
        
        let mut res = DiffussionSolver::solve_by_thomas(&a, &b).unwrap();
        
        let len = res.len();
        res[0] = dif_solver.boundary_conditions[0];
        res[len-1] = dif_solver.boundary_conditions[1];

        assert!(res.len() == 5);
        assert!(res[1] >= 0.15 && res[1] <= 0.17);
        assert!(res[2] >= 0.36 && res[2] <= 0.38);
        assert!(res[3] >= 0.60 && res[3] <= 0.68);
    }
}
