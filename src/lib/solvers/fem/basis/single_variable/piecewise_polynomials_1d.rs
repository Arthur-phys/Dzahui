use crate::{Error, solvers::fem::basis::single_variable::polynomials_1d::FirstDegreePolynomial};

pub struct PiecewiseFirstDegreePolynomial {
    coefficients: Vec<f64>,
    independent_terms: Vec<f64>,
    interval_breakpoints: Vec<f64>
}

impl PiecewiseFirstDegreePolynomial {

    pub fn constants(independent_terms: Vec<f64>, interval_breakpoints: Vec<f64>) -> Result<Self,Error> {
        
        if independent_terms.len() != interval_breakpoints.len() + 1 {
            return Err(Error::PieceWiseDims);
        }

        Ok(Self {
            coefficients: vec![0_f64; independent_terms.len()],
            independent_terms,
            interval_breakpoints
        })
    }

    pub fn from_polynomials(polynomials: Vec<FirstDegreePolynomial>, interval_breakpoints: Vec<f64>) -> Result<Self,Error> {

        if polynomials.len() != interval_breakpoints.len() + 1 {
            return Err(Error::PieceWiseDims);
        }

        let mut coefficients = Vec::with_capacity(polynomials.len());
        let mut independent_terms = Vec::with_capacity(polynomials.len());

        for (i,pol) in polynomials.iter().enumerate() {
            coefficients[i] = pol.coefficient;
            independent_terms[i] = pol.independent_term;
        }

        Ok(Self {
            coefficients,
            independent_terms,
            interval_breakpoints
        })
    
    }

    pub fn evaluate_on_interval(&self, n: usize, x: f64) -> f64 {
        
        if n >= self.coefficients.len() {
            panic!("Evaluation out of bounds. No defined function for interval {}",n);
        }
        self.coefficients[n] * x + self.independent_terms[n]
    
    }

}