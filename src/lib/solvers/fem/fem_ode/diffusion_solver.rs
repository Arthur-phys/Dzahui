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

trait Function {
    fn evaluate(&self, x: f32) -> f32;
}

trait Affine: Function {
    fn compose(&self, other: Self) -> Self;
}

trait Differentiable: Function {
    fn differentiate(&self) -> Box<dyn Function>;
}

struct FirstDegreePolynomial {
    coefficient: f32,
    independent_term: f32,
}

impl FirstDegreePolynomial {
    
    fn new(coefficient: f32, independent_term: f32) -> Self {
        Self {
            coefficient,
            independent_term
        }
    }

    fn zero() -> Self {
        Self {
            coefficient: 0_f32,
            independent_term: 0_f32
        }
    }

    fn constant(independent_term: f32) -> Self {
        Self {
            coefficient: 0_f32,
            independent_term
        }
    }
}


impl Function for FirstDegreePolynomial {
    
    fn evaluate(&self, x: f32) -> f32 {
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

impl Affine for FirstDegreePolynomial {
    fn compose(&self, other: Self) -> Self {
        Self {
            coefficient: self.coefficient * other.coefficient,
            independent_term: self.coefficient * other.independent_term + self.independent_term
        }
    }
}

trait IntervalStep {}
trait NumberOfArguments {}

impl IntervalStep for [f32;3] {}
impl NumberOfArguments for [f32;4] {}

struct PiecewiseFirstDegreePolynomial<A: IntervalStep, B: NumberOfArguments> {
    coefficients: B,
    independent_terms: B,
    interval: A
}

impl PiecewiseFirstDegreePolynomial<[f32;3],[f32;4]> {
    
    fn new(coefficients: [f32;4], independent_terms: [f32;4], interval: [f32;3]) -> Self {
        Self {
            coefficients,
            independent_terms,
            interval
        }
    }

    fn constants(independent_terms: [f32;4], interval: [f32;3]) -> Self {
        Self {
            coefficients: [0_f32;4],
            independent_terms,
            interval  
        }
    }

    fn from_polynomials(functions: [&FirstDegreePolynomial;4],interval: [f32;3]) -> Self {
        Self {
            coefficients: [functions[0].coefficient,functions[1].coefficient,
            functions[2].coefficient,functions[3].coefficient],
            independent_terms: [functions[0].independent_term,functions[1].independent_term,
            functions[2].independent_term,functions[3].independent_term],
            interval
        }
    }

}

impl Function for PiecewiseFirstDegreePolynomial<[f32;3],[f32;4]> {
    
    fn evaluate(&self, x: f32) -> f32 {
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

impl Differentiable for PiecewiseFirstDegreePolynomial<[f32;3],[f32;4]> {
    
    fn differentiate(&self) -> Box<dyn Function> {
        Box::new(
            PiecewiseFirstDegreePolynomial::constants(self.independent_terms, self.interval)
        )
    }

}

struct TransformationFactory();

impl TransformationFactory {

    fn build(&self, beg: f32, end: f32) -> FirstDegreePolynomial {
        let coefficient = 1_f32 / (end - beg);
        let independent_term = -1_f32 / (end - beg); 
        FirstDegreePolynomial { coefficient, independent_term }
    }

}

struct LinearBasis {
    basis: Vec<FirstDegreePolynomial>,
    transformation: TransformationFactory
}

struct PiecewiseLinearBasis<A: IntervalStep, B: NumberOfArguments> {
    basis: Vec<PiecewiseFirstDegreePolynomial<A,B>>,
    interval: Vec<f32>,
}

impl<A: IntervalStep, B: NumberOfArguments> PiecewiseLinearBasis<A, B> { 
    fn new(basis: Vec<PiecewiseFirstDegreePolynomial<A,B>>, interval: Vec<f32>) -> Self {
        Self {
            basis,
            interval
        }
    }
}

impl LinearBasis {

    fn new() -> Self {
        let phi_1 = FirstDegreePolynomial::new(1_f32,0_f32);
        let phi_2 = FirstDegreePolynomial::new(-1_f32,1_f32);
        Self {
            basis: vec![phi_1,phi_2],
            transformation: TransformationFactory()
        }
    }

    fn transform_basis(self, mesh: Vec<f32>) -> PiecewiseLinearBasis<[f32;3],[f32;4]> {

        let phi_1 = &self.basis[0];
        let phi_2= &self.basis[1];

        let zero = FirstDegreePolynomial::zero();

        let transformation = self.transformation.build(mesh[0],mesh[1]);
        let mut basis_transform = phi_1.compose(transformation);

        let basis_vec = vec![
            PiecewiseFirstDegreePolynomial::from_polynomials([&zero,&zero,&basis_transform,&zero], [mesh[0]-1_f32,mesh[0],mesh[1]])
        ];

        mesh.iter().zip(mesh.iter().skip(1)).zip(mesh.iter().skip(2)).for_each(|((prev, cur), next)| {

        });

        PiecewiseLinearBasis::new(basis_vec,mesh)
    }
} 

struct DiffussionSolver {
    boundary_conditions: [f32; 2],
    vertices: Vec<f32>,
}

impl DiffussionSolver {

    fn new(boundary_conditions: [f32; 2], vertices: Vec<f32>) -> Self {
        Self {
            boundary_conditions,
            vertices
        }
    }
}

