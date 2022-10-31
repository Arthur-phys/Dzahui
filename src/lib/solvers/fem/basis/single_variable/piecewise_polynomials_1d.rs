use crate::{solvers::fem::basis::single_variable::polynomials_1d::FirstDegreePolynomial, Error};

use super::{Differentiable1D, Function1D};

#[derive(PartialEq, Debug)]
pub struct PiecewiseFirstDegreePolynomial {
    polynomials: Vec<FirstDegreePolynomial>,
    interval_breakpoints: Vec<f64>,
}

impl PiecewiseFirstDegreePolynomial {
    pub fn from_values(
        coefficients: Vec<f64>,
        independent_terms: Vec<f64>,
        interval_breakpoints: Vec<f64>,
    ) -> Result<Self, Error> {
        if independent_terms.len() != interval_breakpoints.len() + 1
            || independent_terms.len() != coefficients.len()
        {
            return Err(Error::PieceWiseDims);
        }

        let polynomials = coefficients
            .into_iter()
            .zip(independent_terms)
            .map(|(coef, i_term)| -> FirstDegreePolynomial {
                FirstDegreePolynomial::new(coef, i_term)
            })
            .collect();

        Ok(Self {
            polynomials,
            interval_breakpoints,
        })
    }

    pub fn from_constants(
        independent_terms: Vec<f64>,
        interval_breakpoints: Vec<f64>,
    ) -> Result<Self, Error> {
        if independent_terms.len() != interval_breakpoints.len() + 1 {
            return Err(Error::PieceWiseDims);
        }

        let polynomials = independent_terms
            .into_iter()
            .map(|i_term| -> FirstDegreePolynomial { FirstDegreePolynomial::new(0_f64, i_term) })
            .collect();

        Ok(Self {
            polynomials,
            interval_breakpoints,
        })
    }

    pub fn from_polynomials(
        polynomials: Vec<FirstDegreePolynomial>,
        interval_breakpoints: Vec<f64>,
    ) -> Result<Self, Error> {
        if polynomials.len() != interval_breakpoints.len() + 1 {
            return Err(Error::PieceWiseDims);
        }

        Ok(Self {
            polynomials,
            interval_breakpoints,
        })
    }
}

impl Function1D for PiecewiseFirstDegreePolynomial {
    fn evaluate(&self, x: f64) -> f64 {
        let val = self.interval_breakpoints.iter().enumerate().find_map(
            |(i, breakpoint)| -> Option<f64> {
                if x < *breakpoint {
                    Some(self.polynomials[i].evaluate(x))
                } else {
                    None
                }
            },
        );

        match val {
            Some(num) => num,
            None => self.polynomials[self.interval_breakpoints.len()].evaluate(x),
        }
    }
}

impl Differentiable1D<PiecewiseFirstDegreePolynomial> for PiecewiseFirstDegreePolynomial {
    fn differentiate(&self) -> PiecewiseFirstDegreePolynomial {
        let diff_polynomials = self
            .polynomials
            .iter()
            .map(|pol| -> FirstDegreePolynomial { pol.differentiate() })
            .collect();

        match PiecewiseFirstDegreePolynomial::from_polynomials(
            diff_polynomials,
            self.interval_breakpoints.clone(),
        ) {
            Ok(diff) => diff,
            Err(e) => panic!("{}", e),
        }
    }
}
