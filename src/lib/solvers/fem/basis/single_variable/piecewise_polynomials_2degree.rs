
use crate::Error;

use super::{polynomials_1d::{SecondDegreePolynomial, FirstDegreePolynomial}, piecewise_polynomials_1degree::PiecewiseFirstDegreePolynomial};
use crate::solvers::basis::functions::{Function1D,Differentiable1D};
/// # General Information
///
/// A piecewise definition of a first-degree polynomial function. Carries both a vector of functions and the intervals on which each must be evaluated.
/// It is always supposed the points in the interval are in ascending order. Giving the function in any other order will result in erratic behaviour.
///
/// # Fields
///
/// * `polynomials` - A vector of first-degree polynomials. Must be the same length as `interval_breakpoints + 1`.
/// * `interval_breakpoints` - A vector of 1D points in ascending order to know which function to evaluate. Must be the same length as `polynomials - 1`
///
#[derive(PartialEq, Debug)]
pub struct PiecewiseSecondDegreePolynomial {
    polynomials: Vec<SecondDegreePolynomial>,
    interval_breakpoints: Vec<f64>,
}

impl PiecewiseSecondDegreePolynomial {
    /// # General Information
    ///
    /// Creates a new instance from raw values for coefficients and independent terms.
    ///
    /// # Parameters
    ///
    /// * `quadratic_coefficients` - Values that multiply a quadratic variable.
    /// * `linear_coefficients` - Values that miltiply a linear variable.
    /// * `independent_terms` - Values that are added to variable.
    /// * `interval_breakpoints` - Points in ascending order to know which function to evaluate.
    ///
    pub fn from_values<A: IntoIterator<Item = f64>, B: IntoIterator<Item = f64>>(
        quadratic_coefficients: A,
        linear_coefficients: A,
        independent_terms: A,
        interval_breakpoints: B,
    ) -> Result<Self, Error> {

        let independent_terms: Vec<f64> = independent_terms.into_iter().collect();
        let linear_coefficients: Vec<f64> = linear_coefficients.into_iter().collect();
        let quadratic_coefficients: Vec<f64> = quadratic_coefficients.into_iter().collect();
        let interval_breakpoints: Vec<f64> = interval_breakpoints.into_iter().collect();

        if independent_terms.len() != interval_breakpoints.len() + 1
            || independent_terms.len() != linear_coefficients.len()
            || independent_terms.len() != quadratic_coefficients.len()
            || linear_coefficients.len() != independent_terms.len() 
        {
            return Err(Error::PieceWiseDims);
        }

        let polynomials = quadratic_coefficients
            .into_iter()
            .zip(linear_coefficients)
            .zip(independent_terms)
            .map(|((quad_coef, lin_coef), i_term)| -> SecondDegreePolynomial {
                SecondDegreePolynomial::new(quad_coef,lin_coef,i_term)
            })
            .collect();

        Ok(Self {
            polynomials,
            interval_breakpoints,
        })
    }

    /// # General Information
    ///
    /// Given a vector of polynomials, creates a piecewise function with all of them.
    ///
    /// # Parameters
    ///
    /// * `polynomials` - A vector with all the polynomials to use for piecewise definition.
    /// * `interval_breakpoints` - Points in ascending order to know which function to evaluate.
    ///
    pub fn from_polynomials<A: IntoIterator<Item = SecondDegreePolynomial>, B: IntoIterator<Item = f64>>(
        polynomials: A,
        interval_breakpoints: B,
    ) -> Result<Self, Error> {

        let polynomials: Vec<SecondDegreePolynomial> = polynomials.into_iter().collect();
        let interval_breakpoints: Vec<f64> = interval_breakpoints.into_iter().collect();

        if polynomials.len() != interval_breakpoints.len() + 1 {
            return Err(Error::PieceWiseDims);
        }

        Ok(Self {
            polynomials,
            interval_breakpoints,
        })
    }
}

impl Function1D for PiecewiseSecondDegreePolynomial {
    /// # Specific implementation
    ///
    /// **Remember that number of functions = number of breakpoints + 1**.
    /// Evaluates the function supposing that `interval_breakpoints` is in ascending order.
    /// Every breakpoint coincides with a function (except for the last one). That is, given the breakpoint vector index i,
    /// breakpoint i coincides with function i.
    /// Evaluation is made via checking if variable `x` is less than current breakpoint. If x is bigger than every breakpoint, then the last function is
    /// evaluated.
    ///
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

impl Differentiable1D<PiecewiseFirstDegreePolynomial> for PiecewiseSecondDegreePolynomial {
    /// # Specific implementation
    ///
    /// The derivative of a piecewise second degree polynomial is a pieewise first degree polynomial.
    /// Resulting function is obtained via differentiation of every second degree polynomial in instance.
    ///
    fn differentiate(&self) -> Result<PiecewiseFirstDegreePolynomial,Error> {
        let diff_polynomials: Vec<FirstDegreePolynomial> = self
            .polynomials
            .iter()
            .map(|pol| -> Result<FirstDegreePolynomial,Error> { pol.differentiate() })
            .collect::<Result<Vec<FirstDegreePolynomial>,_>>()?;

        PiecewiseFirstDegreePolynomial::from_polynomials(
            diff_polynomials,
            self.interval_breakpoints.clone()
        )
    }
}
