pub mod linear_basis;

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