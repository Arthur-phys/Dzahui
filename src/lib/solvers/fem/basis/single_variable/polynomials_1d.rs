use crate::Error;

// Internal dependencies.
use super::functions::{Composable1D, Differentiable1D, Function1D};

#[derive(PartialEq, Debug)]
/// # General Information
///
/// A simple, first degree polynomial in one variable.
///
/// # Fields
///
/// * `coefficient` - constant that multiplies variable.
/// * `independent_term` - constant that adds to variable.
///
pub struct FirstDegreePolynomial {
    pub(crate) coefficient: f64,
    pub(crate) independent_term: f64,
}

#[derive(PartialEq, Debug)]
/// # General Information
///
/// A simple, second degree polynomial in one variablee
///
/// # Fields
///
/// * `quadratic_coefficient` - constant that multiplies quadratic term.
/// * `linear_coefficient` - constant that multiplies linear term.
/// * `independent_term` - constant that adds to varaible.
///
pub struct SecondDegreePolynomial {
    quadratic_coefficient: f64,
    linear_coefficient: f64,
    independent_term: f64,
}

impl FirstDegreePolynomial {
    /// Normal constructor.
    pub fn new(coefficient: f64, independent_term: f64) -> FirstDegreePolynomial {
        FirstDegreePolynomial {
            coefficient,
            independent_term,
        }
    }

    /// Zero function factory.
    pub fn zero() -> FirstDegreePolynomial {
        Self {
            coefficient: 0_f64,
            independent_term: 0_f64,
        }
    }

    /// Constant function factory.
    pub fn constant(independent_term: f64) -> FirstDegreePolynomial {
        Self {
            coefficient: 0_f64,
            independent_term,
        }
    }

    /// Transformation from any interval to [0,1].
    pub fn transformation_to_0_1(beg: f64, end: f64) -> FirstDegreePolynomial {
        let coefficient = 1_f64 / (end - beg);
        let independent_term = -beg / (end - beg);
        FirstDegreePolynomial {
            coefficient,
            independent_term,
        }
    }

    /// Transformation from [-1,1] to any interval.
    pub fn transformation_from_m1_p1(beg: f64, end: f64) -> FirstDegreePolynomial {
        let coefficient = (end - beg) / 2_f64;
        let independent_term = (end + beg) / 2_f64;
        FirstDegreePolynomial {
            coefficient,
            independent_term,
        }
    }

    // One of two basis functions on unit interval [0,1]
    pub fn phi_1() -> FirstDegreePolynomial {
        FirstDegreePolynomial {
            coefficient: 1_f64,
            independent_term: 0_f64,
        }
    }

    // One of two basis functions on unit interval [0,1]
    pub fn phi_2() -> FirstDegreePolynomial {
        FirstDegreePolynomial {
            coefficient: -1_f64,
            independent_term: 1_f64,
        }
    }
}

impl Function1D for FirstDegreePolynomial {
    /// # Specific Implementation
    ///
    /// Simple evaluation of a polynomial.
    ///
    fn evaluate(&self, x: f64) -> f64 {
        self.coefficient * x + self.independent_term
    }
}

// self(other(x))
impl Composable1D<FirstDegreePolynomial, FirstDegreePolynomial> for FirstDegreePolynomial {
    /// # Specific Implementation
    ///
    /// Composition of two first degree polynomials results in another polynomial.
    ///
    fn compose(self, other: FirstDegreePolynomial) -> Result<FirstDegreePolynomial,Error> {
        Ok(FirstDegreePolynomial {
            coefficient: self.coefficient * other.coefficient,
            independent_term: self.coefficient * other.independent_term + self.independent_term,
        })
    }
}

impl Differentiable1D<FirstDegreePolynomial> for FirstDegreePolynomial {
    /// # Specific Implementation
    ///
    /// Differentiation of a first degree polynomial results in a constant.
    ///
    fn differentiate(&self) -> Result<FirstDegreePolynomial,Error> {
        Ok(FirstDegreePolynomial {
            coefficient: 0_f64,
            independent_term: self.coefficient,
        })
    }
}

impl SecondDegreePolynomial {
    /// Simple constructor for second degree polynomial.
    pub fn new(
        quadratic_coefficient: f64,
        linear_coefficient: f64,
        independent_term: f64,
    ) -> SecondDegreePolynomial {
        SecondDegreePolynomial {
            quadratic_coefficient,
            linear_coefficient,
            independent_term,
        }
    }
}

impl Function1D for SecondDegreePolynomial {
    /// # Specific Implementation
    ///
    /// Simple evaluation of a second degree polynomial.
    ///
    fn evaluate(&self, x: f64) -> f64 {
        self.quadratic_coefficient * x.powf(2_f64)
            + self.linear_coefficient * x
            + self.independent_term
    }
}

impl Differentiable1D<FirstDegreePolynomial> for SecondDegreePolynomial {
    /// # Specific Implementation
    ///
    /// Differentiation of a second degree polynomial results on a first degree polynomial.
    ///
    fn differentiate(&self) -> Result<FirstDegreePolynomial,Error> {
        Ok(FirstDegreePolynomial {
            coefficient: 2_f64 * self.quadratic_coefficient,
            independent_term: self.linear_coefficient,
        })
    }
}

// self(other(x))
impl Composable1D<FirstDegreePolynomial, SecondDegreePolynomial> for SecondDegreePolynomial {
    /// # Specific implementation
    ///
    /// Composing a second degree polynomial with a first degree polynomial gives another second degree polynomial.
    ///
    fn compose(self, other: FirstDegreePolynomial) -> Result<SecondDegreePolynomial,Error> {
        Ok(SecondDegreePolynomial {
            quadratic_coefficient: self.quadratic_coefficient * other.coefficient.powf(2_f64),
            linear_coefficient: 2_f64
                * self.quadratic_coefficient
                * other.coefficient
                * other.independent_term
                + self.linear_coefficient * other.coefficient,
            independent_term: other.independent_term.powf(2_f64)
                + other.independent_term * self.linear_coefficient
                + self.independent_term,
        })
    }
}
