// use super::{Vertex1D, BoundaryVertex1D, PolynomialDegree, DiffEquationSolver};

// struct DiffussionSolverBuilder {
//     boundary_vertices: [BoundaryVertex1D; 2],
//     vertices: Option<Vec<Vertex1D>>,
//     polynomial_degree: Option<PolynomialDegree>,
// }

// impl DiffussionSolverBuilder {
//     fn build(self) -> DiffussionSolver {
//         todo!()
//     }
// }

use ndarray::{Array, Ix2, Ix1, Array1, array};

use crate::solvers::fem::fem_ode::quadrature1d::gauss_legendre::GaussLegendreQuadrature;


pub trait Function {
    fn evaluate(&self, x: f64) -> f64;
}

pub trait Affine: Function {
    fn compose(&self, other: Self) -> Self;
}

pub trait Differentiable: Function {
    fn differentiate(&self) -> Box<dyn Function>;
}

pub(crate) struct FirstDegreePolynomial {
    pub(crate) coefficient: f64,
    pub(crate) independent_term: f64,
}

impl FirstDegreePolynomial {
    
    fn new(coefficient: f64, independent_term: f64) -> Self {
        Self {
            coefficient,
            independent_term
        }
    }

    fn zero() -> Self {
        Self {
            coefficient: 0_f64,
            independent_term: 0_f64
        }
    }

    fn constant(independent_term: f64) -> Self {
        Self {
            coefficient: 0_f64,
            independent_term
        }
    }
}

impl Function for FirstDegreePolynomial {
    
    fn evaluate(&self, x: f64) -> f64 {
        self.coefficient * x + self.independent_term
    }

}

impl Differentiable for FirstDegreePolynomial {
    
    fn differentiate(&self) -> Box<dyn Function> {
        Box::new(
            FirstDegreePolynomial::constant(self.coefficient)
        )
    }

}

impl PartialEq for FirstDegreePolynomial {
    fn eq(&self, other: &Self) -> bool {
        self.coefficient == other.coefficient && self.independent_term == other.independent_term
    }
}

impl Eq for FirstDegreePolynomial {}

impl Affine for FirstDegreePolynomial {
    fn compose(&self, other: Self) -> Self {
        Self {
            coefficient: self.coefficient * other.coefficient,
            independent_term: self.coefficient * other.independent_term + self.independent_term
        }
    }
}

pub(crate) trait IntervalStep {}
pub(crate) trait NumberOfArguments {}

impl IntervalStep for [f64;3] {}
impl NumberOfArguments for [f64;4] {}

#[derive(Debug)]
pub(crate) struct PiecewiseFirstDegreePolynomial<A: IntervalStep, B: NumberOfArguments> {
    coefficients: B,
    independent_terms: B,
    interval: A
}

impl PiecewiseFirstDegreePolynomial<[f64;3],[f64;4]> {
    
    fn new(coefficients: [f64;4], independent_terms: [f64;4], interval: [f64;3]) -> Self {
        Self {
            coefficients,
            independent_terms,
            interval
        }
    }

    fn constants(independent_terms: [f64;4], interval: [f64;3]) -> Self {
        Self {
            coefficients: [0_f64;4],
            independent_terms,
            interval
        }
    }

    fn from_polynomials(functions: [&FirstDegreePolynomial;4],interval: [f64;3]) -> Self {
        Self {
            coefficients: [functions[0].coefficient,functions[1].coefficient,
            functions[2].coefficient,functions[3].coefficient],
            independent_terms: [functions[0].independent_term,functions[1].independent_term,
            functions[2].independent_term,functions[3].independent_term],
            interval
        }
    }

}

impl Function for PiecewiseFirstDegreePolynomial<[f64;3],[f64;4]> {
    
    fn evaluate(&self, x: f64) -> f64 {
        if x < self.interval[0] {
            self.coefficients[0] * x + self.independent_terms[0]
        } else if x >= self.interval[0] && x < self.interval[1] {
            self.coefficients[1] * x + self.independent_terms[1]
        } else if x >= self.interval[1] && x < self.interval[2] {
            self.coefficients[2] * x + self.independent_terms[2]
        } else {
            self.coefficients[3] * x + self.independent_terms[3]
        }
    }

}

impl PartialEq for PiecewiseFirstDegreePolynomial<[f64;3],[f64;4]> {
    fn eq(&self, other: &Self) -> bool {
        self.coefficients == other.coefficients && self.independent_terms == other.independent_terms && self.interval == other.interval
    }
}

impl Eq for PiecewiseFirstDegreePolynomial<[f64;3],[f64;4]> {}

impl Differentiable for PiecewiseFirstDegreePolynomial<[f64;3],[f64;4]> {
    
    fn differentiate(&self) -> Box<dyn Function> {
        Box::new(
            PiecewiseFirstDegreePolynomial::constants(self.coefficients, self.interval)
        )
    }

}

pub(crate) struct TransformationFactory();

impl TransformationFactory {

    pub fn build(&self, beg: f64, end: f64) -> FirstDegreePolynomial {
        let coefficient = 1_f64 / (end - beg);
        let independent_term = - beg / (end - beg); 
        FirstDegreePolynomial { coefficient, independent_term }
    }

    pub fn build_to_m1_p1(&self, beg: f64, end: f64) -> FirstDegreePolynomial {
        let coefficient = (end - beg) / 2_f64;
        let independent_term = (end + beg) / 2_f64;
        FirstDegreePolynomial { coefficient, independent_term }
    }

}

struct LinearBasis {
    basis: Vec<FirstDegreePolynomial>,
    transformation: TransformationFactory
}

struct PiecewiseLinearBasis<A: IntervalStep, B: NumberOfArguments> {
    basis: Vec<PiecewiseFirstDegreePolynomial<A,B>>,
    transformation: TransformationFactory
}

impl<A: IntervalStep, B: NumberOfArguments> PiecewiseLinearBasis<A, B> { 
    fn new(basis: Vec<PiecewiseFirstDegreePolynomial<A,B>>) -> Self {
        Self {
            basis,
            transformation: TransformationFactory()
        }
    }
}

impl LinearBasis {

    fn new_unit() -> Self {
        let phi_1 = FirstDegreePolynomial::new(1_f64,0_f64);
        let phi_2 = FirstDegreePolynomial::new(-1_f64,1_f64);
        Self {
            basis: vec![phi_1,phi_2],
            transformation: TransformationFactory()
        }
    }

    fn transform_basis(self, mesh: &Vec<f64>) -> PiecewiseLinearBasis<[f64;3],[f64;4]> {

        let phi_1 = &self.basis[0];
        let phi_2= &self.basis[1];

        let zero = FirstDegreePolynomial::zero();

        let transformation = self.transformation.build(mesh[0],mesh[1]);
        let initial_transform_function = phi_2.compose(transformation);

        let mut basis_vec = vec![
            PiecewiseFirstDegreePolynomial::from_polynomials([&zero,&zero,&initial_transform_function,&zero], [mesh[0]-1_f64,mesh[0],mesh[1]])
        ];

        mesh.iter().zip(mesh.iter().skip(1)).zip(mesh.iter().skip(2)).for_each(|((prev, cur), next)| {
            
            let transformation = self.transformation.build(*prev,*cur);
            let basis_left = phi_1.compose(transformation);
            let transformation = self.transformation.build(*cur, *next);
            let basis_right = phi_2.compose(transformation);
            
            basis_vec.push(
                PiecewiseFirstDegreePolynomial::from_polynomials([&zero,&basis_left,&basis_right,&zero], [*prev,*cur,*next])
            )
        });
        
        let transformation = self.transformation.build(mesh[mesh.len()-2],mesh[mesh.len()-1]);
        let final_transform_function = phi_1.compose(transformation);

        basis_vec.push(
            PiecewiseFirstDegreePolynomial::from_polynomials([&zero,&final_transform_function,&zero,&zero],
                 [mesh[mesh.len()-2],mesh[mesh.len()-1],mesh[mesh.len()-1] + 1_f64])
        );

        PiecewiseLinearBasis::new(basis_vec)
    }
}

struct DiffussionSolver {
    boundary_conditions: [f64; 2],
    mesh: Vec<f64>,
    mu: f64,
    b: f64
}

impl DiffussionSolver {

    fn new(boundary_conditions: [f64; 2], mesh: Vec<f64>, mu: f64, b: f64) -> Self {
        Self {
            boundary_conditions,
            mesh,
            mu,
            b
        }
    }

    fn obtain_dirichlet_homogeneous_linear_system(&self, gauss_step_number: usize) -> (Array<f64,Ix2>,Array<f64,Ix1>) {

        let (odd_theta_zeros,even_theta_zeros,odd_weights,even_weights) = GaussLegendreQuadrature::load_tabulated_values();
        let linear_unit = LinearBasis::new_unit();
        let basis = linear_unit.transform_basis(&self.mesh);
        let long_basis = basis.basis.len();

        let mut stiffness_matrix = ndarray::Array::from_elem((long_basis-2,long_basis-2),0_f64);

        if long_basis-2 == 1 {
            let derivative_phi = basis.basis[1].differentiate();
            let transform_function = basis.transformation.build_to_m1_p1(self.mesh[0], self.mesh[2]);
            let derivative_t = transform_function.differentiate();
            let mut integral_square_approximation = 0_f64;

            for j in 1..gauss_step_number {
    
                // Obtaining arccos(node) and weight
                let (theta, w) = GaussLegendreQuadrature::quad_pair(gauss_step_number,j,&odd_theta_zeros,&even_theta_zeros,&odd_weights,&even_weights);
                let x = theta.cos();

                // translated to -1,1
                let translated_point_square = transform_function.evaluate(x);

                integral_square_approximation +=  (self.mu * derivative_phi.evaluate(translated_point_square) * derivative_phi.evaluate(translated_point_square) + self.b * derivative_phi.evaluate(translated_point_square) * basis.basis[1].evaluate(translated_point_square)) * derivative_t.evaluate(x) * w;   
            }

            stiffness_matrix[[0,0]] = integral_square_approximation;
            
        } else {
             
            if long_basis-2 == 2 {

            } else {

                for i in 2..long_basis-2 {
        
                    let derivative_phi = basis.basis[i].differentiate();
        
                    let transform_function_prev = basis.transformation.build_to_m1_p1(self.mesh[i-1], self.mesh[i]);
                    let transform_function_next = basis.transformation.build_to_m1_p1(self.mesh[i], self.mesh[i+1]);
                    let transform_function_square = basis.transformation.build_to_m1_p1(self.mesh[i-1], self.mesh[i+1]);
                    let derivative_t_prev = transform_function_prev.differentiate();
                    let derivative_t_next = transform_function_next.differentiate();
                    let derivative_t_square = transform_function_square.differentiate();
        
                    let derivative_prev = basis.basis[i-1].differentiate();
                    let derivative_next = basis.basis[i+1].differentiate();
        
                    let mut integral_prev_approximation = 0_f64;
                    let mut integral_next_approximation = 0_f64;
                    let mut integral_square_approximation = 0_f64;
        
                    for j in 1..gauss_step_number {
        
                        // Obtaining arccos(node) and weight
                        let (theta, w) = GaussLegendreQuadrature::quad_pair(gauss_step_number,j,&odd_theta_zeros,&even_theta_zeros,&odd_weights,&even_weights);
                        let x = theta.cos();
        
                        // translated to -1,1
                        let translated_point_prev = transform_function_prev.evaluate(x);
                        let translated_point_next = transform_function_next.evaluate(x);
                        let translated_point_square = transform_function_square.evaluate(x);
        
                        integral_prev_approximation +=  (self.mu * derivative_phi.evaluate(translated_point_prev) * derivative_prev.evaluate(translated_point_prev) + self.b * derivative_phi.evaluate(translated_point_prev) * basis.basis[i-1].evaluate(translated_point_prev)) * derivative_t_prev.evaluate(x) * w;
                        integral_next_approximation +=  (self.mu * derivative_phi.evaluate(translated_point_next) * derivative_next.evaluate(translated_point_next) + self.b * derivative_phi.evaluate(translated_point_next) * basis.basis[i+1].evaluate(translated_point_next)) * derivative_t_next.evaluate(x) * w;
                        integral_square_approximation +=  (self.mu * derivative_phi.evaluate(translated_point_square) * derivative_phi.evaluate(translated_point_square) + self.b * derivative_phi.evaluate(translated_point_square) * basis.basis[i].evaluate(translated_point_square)) * derivative_t_square.evaluate(x) * w;
                        
                    }
                    stiffness_matrix[[i-1,i-2]] = integral_next_approximation;
                    stiffness_matrix[[i-1,i]] = integral_prev_approximation;
                    stiffness_matrix[[i-1,i-1]] = integral_square_approximation;
                }
            }


            // elements here are special cases which only present a single bilinear evaluation either on the right or on the left
            let derivative_phi_zero_internal = basis.basis[1].differentiate();
            let derivative_phi_last_internal = basis.basis[long_basis-2].differentiate();

            let transform_function_zero_internal = basis.transformation.build_to_m1_p1(self.mesh[1], self.mesh[2]);
            let transform_function_zero_square = basis.transformation.build_to_m1_p1(self.mesh[0], self.mesh[2]);
            let transform_function_last_internal = basis.transformation.build_to_m1_p1(self.mesh[long_basis-3], self.mesh[long_basis-2]);
            let transform_function_last_square = basis.transformation.build_to_m1_p1(self.mesh[long_basis-3], self.mesh[long_basis-1]);

            let derivative_t_zero = transform_function_zero_internal.differentiate();
            let derivative_t_last = transform_function_last_internal.differentiate();
            let derivative_t_zq = transform_function_zero_square.differentiate();
            let derivative_t_lq = transform_function_last_square.differentiate();

            let derivative_one_internal = basis.basis[2].differentiate();
            let derivative_pen_internal = basis.basis[long_basis-3].differentiate();

            let mut integral_zero_internal_square_approximation = 0_f64;
            let mut integral_zero_internal_one_approximation = 0_f64;
            let mut integral_last_internal_square_approximation = 0_f64;
            let mut integral_last_internal_pen_approximation = 0_f64;

            for i in 1..gauss_step_number {

                // Obtaining arccos(node) and weight
                let (theta, w) = GaussLegendreQuadrature::quad_pair(gauss_step_number,i,&odd_theta_zeros,&even_theta_zeros,&odd_weights,&even_weights);
                let x = theta.cos();

                // translated to original interval
                let translated_point_zero = transform_function_zero_internal.evaluate(x);
                let translated_point_zs = transform_function_zero_square.evaluate(x); 
                let translated_point_last = transform_function_last_internal.evaluate(x);
                let translated_point_ls = transform_function_last_square.evaluate(x);

                integral_zero_internal_square_approximation +=  (self.mu * derivative_phi_zero_internal.evaluate(translated_point_zs) * derivative_phi_zero_internal.evaluate(translated_point_zs) + self.b * derivative_phi_zero_internal.evaluate(translated_point_zs) * basis.basis[1].evaluate(translated_point_zs)) * derivative_t_zq.evaluate(x) * w;
                integral_zero_internal_one_approximation +=  (self.mu * derivative_phi_zero_internal.evaluate(translated_point_zero) * derivative_one_internal.evaluate(translated_point_zero) + self.b * derivative_one_internal.evaluate(translated_point_zero) * basis.basis[1].evaluate(translated_point_zero)) * derivative_t_zero.evaluate(x) * w;
                integral_last_internal_square_approximation +=  (self.mu * derivative_phi_last_internal.evaluate(translated_point_ls) * derivative_phi_last_internal.evaluate(translated_point_ls) + self.b * derivative_phi_last_internal.evaluate(translated_point_ls) * basis.basis[long_basis-2].evaluate(translated_point_ls)) * derivative_t_lq.evaluate(x) * w;
                integral_last_internal_pen_approximation +=  (self.mu * derivative_phi_last_internal.evaluate(translated_point_last) * derivative_pen_internal.evaluate(translated_point_last) + self.b * derivative_pen_internal.evaluate(translated_point_last) * basis.basis[long_basis-2].evaluate(translated_point_last)) * derivative_t_last.evaluate(x) * w;
            }
            
            stiffness_matrix[[0,0]] = integral_zero_internal_square_approximation;
            stiffness_matrix[[0,1]] = integral_zero_internal_one_approximation;
            stiffness_matrix[[long_basis-3,long_basis-3]] = integral_last_internal_square_approximation;
            stiffness_matrix[[long_basis-3,long_basis-4]] = integral_last_internal_pen_approximation;
            
        }

        // elements here only serve to impose boundary conditions
        let derivative_phi_zero = basis.basis[0].differentiate();
        let derivative_phi_last = basis.basis[long_basis-1].differentiate();

        let transform_function_zero = basis.transformation.build_to_m1_p1(self.mesh[0], self.mesh[1]);
        let transform_function_last = basis.transformation.build_to_m1_p1(self.mesh[long_basis-2], self.mesh[long_basis-1]);

        let derivative_t_zero = transform_function_zero.differentiate();
        let derivative_t_last = transform_function_last.differentiate();

        let derivative_one_internal = basis.basis[1].differentiate();
        let derivative_pen_internal = basis.basis[long_basis-2].differentiate();

        let mut integral_zero_one_approximation = 0_f64;
        let mut integral_last_pen_approximation = 0_f64;

        for i in 1..gauss_step_number {

            // Obtaining arccos(node) and weight
            let (theta, w) = GaussLegendreQuadrature::quad_pair(gauss_step_number,i,&odd_theta_zeros,&even_theta_zeros,&odd_weights,&even_weights);
            let x = theta.cos();

            // translated to original interval
            let translated_point_zero = transform_function_zero.evaluate(x);
            let translated_point_last = transform_function_last.evaluate(x);

            integral_zero_one_approximation +=  (self.mu * derivative_phi_zero.evaluate(translated_point_zero) * derivative_one_internal.evaluate(translated_point_zero) + self.b * derivative_phi_zero.evaluate(translated_point_zero) * basis.basis[1].evaluate(translated_point_zero)) * derivative_t_zero.evaluate(x) * w;
            integral_last_pen_approximation +=  (self.mu * derivative_phi_last.evaluate(translated_point_last) * derivative_pen_internal.evaluate(translated_point_last) + self.b * derivative_phi_last.evaluate(translated_point_last) * basis.basis[long_basis-2].evaluate(translated_point_last)) * derivative_t_last.evaluate(x) * w;            
        }

        let mut b_vector = Array1::from_elem(long_basis-2, 0_f64);
        b_vector[[0]] += integral_zero_one_approximation * self.boundary_conditions[0];
        b_vector[[long_basis-3]] += integral_last_pen_approximation * self.boundary_conditions[1];
        
        (stiffness_matrix,b_vector) 
        
    }

    fn solve_linear_system(&self, matrix: Array<f64,Ix2>, b: Array<f64,Ix1>) -> Array1<f64> {

        let mut solution = Array1::from_elem(b.len(), 0_f64);
        
        if b.len() == 1 {
            solution[0] = b[0] / matrix[[0,0]];

        } else if b.len() == 2 {
            let det = 1_f64 / (matrix[[0,0]]*matrix[[1,1]] - matrix[[1,0]]*matrix[[0,1]]);
            solution[0] = det * (matrix[[1,1]] * b[0] - matrix[[0,1]] * b[1]);
            solution[1] = det * (-matrix[[1,0]] * b[0] + matrix[[0,0]] * b[1]);

        } else {
            let mut c = Array1::from_elem(b.len()-1, 0_f64);
            let mut d = Array1::from_elem(b.len(), 0_f64);
            c[0] = matrix[[0,1]] / matrix[[0,0]];
            d[0] = b[0] / matrix[[0,0]];

            for i in 1..b.len()-1 {
                c[i] = matrix[[i,i+1]] / (matrix[[i,i]] - matrix[[i,i-1]] * c[i-1]);
                d[i] = (b[i] - matrix[[i,i-1]] * b[i-1]) / (matrix[[i,i]] - matrix[[i,i-1]] * c[i-1]);
            }

            d[b.len()-1] = (b[b.len()-1] - matrix[[b.len()-1,b.len()-2]] * b[b.len()-2]) / (matrix[[b.len()-1,b.len()-1]] - matrix[[b.len()-1,b.len()-2]] * c[b.len()-2]);

            solution[b.len()-1] = d[b.len()-1];
            
            for i in b.len()-2..0 {
                solution[i] = d[i] - c[i] * solution[i+1];
            }
        }

        solution
    }

    fn solve() {
        todo!()
    }
}

#[cfg(test)]
mod test {

    use super::{LinearBasis, FirstDegreePolynomial, PiecewiseFirstDegreePolynomial, DiffussionSolver};
        
    #[test]
    fn create_unit_basis() {
        
        let unit_base = LinearBasis::new_unit();
        let basis = unit_base.basis;
        assert!(basis[0] == FirstDegreePolynomial::new(1_f64,0_f64));
        assert!(basis[1] == FirstDegreePolynomial::new(-1_f64,1_f64));
    }

    #[test]
    fn transform_basis_three_nodes() {

        let unit_base = LinearBasis::new_unit();
        let mesh =vec![0_f64,1_f64,2_f64];
        let transformed = unit_base.transform_basis(&mesh);

        assert!(transformed.basis.len() == 3);

        let first_pol = PiecewiseFirstDegreePolynomial::new([0_f64,0_f64,-1_f64,0_f64], 
            [0_f64,0_f64,1_f64,0_f64], [-1_f64,0_f64,1_f64]);
        let second_pol = PiecewiseFirstDegreePolynomial::new([0_f64,1_f64,-1_f64,0_f64], 
            [0_f64,0_f64,2_f64,0_f64], [0_f64,1_f64,2_f64]);
        let third_pol = PiecewiseFirstDegreePolynomial::new([0_f64,1_f64,0_f64,0_f64], 
            [0_f64,-1_f64,0_f64,0_f64], [1_f64,2_f64,3_f64]);

        assert!(transformed.basis[0] == first_pol);
        assert!(transformed.basis[1] == second_pol);
        assert!(transformed.basis[2] == third_pol);
    }

    #[test]
    fn regular_mesh_matrix_3p() {

        let dif_solver = DiffussionSolver::new([0_f64,1_f64],vec![0_f64,0.5,1_f64],1_f64,1_f64);
        let (a, b) = dif_solver.obtain_dirichlet_homogeneous_linear_system(150);

        assert!(a[[0,0]] <= 4.1 && a[[0,0]] >= 3.9);
        assert!(b[0]>=-1.6 && b[0] <= -1.4);
    }

    #[test]
    fn solve_system_3p() {
        let dif_solver = DiffussionSolver::new([0_f64,1_f64],vec![0_f64,0.5,1_f64],1_f64,1_f64);

        let (a, b) = dif_solver.obtain_dirichlet_homogeneous_linear_system(150);

        let res = dif_solver.solve_linear_system(a, b);

        assert!(res.len() == 1);
        assert!(res[0] <= -0.2 && res[0] >= -0.4);

    }

    #[test]
    fn regular_mesh_matrix_4p() {
        let dif_solver = DiffussionSolver::new([0_f64,1_f64],vec![0_f64,0.33,0.66,1_f64],1_f64,1_f64);
        let (a, b) = dif_solver.obtain_dirichlet_homogeneous_linear_system(150);
        
        assert!(a[[0,0]] <= 6.1 && a[[0,0]] >= 5.9);
        assert!(a[[1,0]] <= -3.4 && a[[1,0]] >= -3.6);
        assert!(a[[0,1]] <= -2.4 && a[[0,1]] >= -2.6);
        assert!(a[[1,1]] <= 6.1 && a[[1,1]] >= 5.9);

        assert!(b[0] == 0_f64);
        assert!(b[1] <= -2.4 && b[1] >= -2.6);
    }

    #[test]
    fn solve_system_4p() {
    
        let dif_solver = DiffussionSolver::new([0_f64,1_f64],vec![0_f64,0.33,0.66,1_f64],1_f64,1_f64);
        let (a, b) = dif_solver.obtain_dirichlet_homogeneous_linear_system(150);

        let res = dif_solver.solve_linear_system(a, b);

        println!("{:?}",res);

        assert!(res.len() == 2);
        assert!(res[0] <= -0.20 && res[0] >= -0.24 );
        assert!(res[1] <= -0.52 && res[1] >= -0.56 );
        
    }

    #[test]
    fn regular_mesh_bigger_matrix() {
        let dif_solver = DiffussionSolver::new([0_f64,1_f64],vec![0_f64,0.25,0.5,0.75,1_f64],1_f64,1_f64);
        let (a, b) = dif_solver.obtain_dirichlet_homogeneous_linear_system(150);

        println!("A: {:?}", a);

        assert!(1==2)

    }

}
