// Internal dependencies.
use super::{polynomials_1d::FirstDegreePolynomial, Differentiable1D, Function1D};
use crate::Error;

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
pub struct PiecewiseFirstDegreePolynomial {
    polynomials: Vec<FirstDegreePolynomial>,
    interval_breakpoints: Vec<f64>,
}

impl PiecewiseFirstDegreePolynomial {
    /// # General Information
    ///
    /// Creates a new instance from raw values for coefficients and independent terms.
    ///
    /// # Parameters
    ///
    /// * `coefficients` - Values that multiply variable.
    /// * `independent_terms` - Values that are added to variable.
    /// * `interval_breakpoints` - Points in ascending order to know which function to evaluate.
    ///
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

    /// # General Information
    ///
    /// Creates a step-like function given a vector of constants.
    ///
    /// # Parameters
    ///
    /// * `independent_terms` - Vector of constants to create function.
    /// * `interval_breakpoints` - Points in ascending order to know which constant to return.
    ///
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

    /// # General Information
    ///
    /// Given a vector of polynomials, creates a piecewise function with all of them.
    ///
    /// # Parameters
    ///
    /// * `polynomials` - A vector with all the polynomials to use for piecewise definition.
    /// * `interval_breakpoints` - Points in ascending order to know which function to evaluate.
    ///
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
    /// # Specific implementation
    ///
    /// **Remember that number of functions = number of breakpoints + 1**.
    /// Evauluates the function supposing that `interval_breakpoints` is in ascending order.
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

impl Differentiable1D<PiecewiseFirstDegreePolynomial> for PiecewiseFirstDegreePolynomial {
    /// # Specific implementation
    ///
    /// The derivative of a piecewise first degree polynomial is a step-like function.
    /// Resulting function is obtained via differentiation of every linear polynomial in instance.
    /// Panic should not be possible.
    ///
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
