// Internal dependencies
use crate::solvers::basis::functions::Function2D;

#[derive(PartialEq, Debug)]
/// # General Information
///
/// A simple, first degree polynomial in two variables.
///
/// # Fields
///
/// * `x_coefficient` - constant that multiplies x variable.
/// * `y_coefficient` - constant that multiplies y variable.
/// * `independent_term` - constant that adds to variable.
///
pub struct FirstDegreePolynomial2D {
    pub(crate) x_coefficient: f64,
    pub(crate) y_coefficient: f64,
    pub(crate) independent_term: f64
}

#[derive(PartialEq, Debug)]
/// # General Information
///
/// A simple, second degree polynomial in two variables
///
/// # Fields
///
/// * `x_quadratic_coefficient` - constant that multiplies x quadratic term.
/// * `y_quadratic_coefficient` - constant that multiplies y quadratic term.
/// * `x_linear_coefficient` - constant that multiplies x linear term.
/// * `y_linear_coefficient` - constant that multiplies y linear term.
/// * `independent_term` - constant that is added to varaibles.
///
pub struct SecondDegreePolynomial2D {
    x_quadratic_coefficient: f64,
    y_quadratic_coefficient: f64,
    xy_coefficient: f64,
    x_linear_coefficient: f64,
    y_linear_coefficient: f64,
    independent_term: f64,
}

impl FirstDegreePolynomial2D {
    /// Normal constructor.
    pub fn new(x_coefficient: f64, y_coefficient: f64, independent_term: f64) -> FirstDegreePolynomial2D {
        FirstDegreePolynomial2D {
            x_coefficient,
            y_coefficient,
            independent_term,
        }
    }

    /// Zero function factory.
    pub fn zero() -> FirstDegreePolynomial2D {
        Self {
            y_coefficient: 0_f64,
            x_coefficient: 0_f64,
            independent_term: 0_f64,
        }
    }

    /// Constant function factory.
    pub fn constant(independent_term: f64) -> FirstDegreePolynomial2D {
        Self {
            y_coefficient: 0_f64,
            x_coefficient: 0_f64,
            independent_term,
        }
    }

    /// Transformation from psi functions into any polynomial function on any triangle.
    /// Must only be used psi_1, psi_2 and psi_3 functions, otherwise it may yield unexpected results
    pub fn transform_original_basis_function(&self, triangle: [(f64,f64);3]) -> FirstDegreePolynomial2D {
        
        let [first,second,third] = triangle;
        let second = (second.0 - first.0, second.1 - first.1);
        let third = (third.0 - first.0, third.1 - first.1);
        let determinant = 1_f64 / (second.0 * third.1 - second.1 * third.0);
        let x_coefficient = determinant * (self.x_coefficient * third.1 - self.y_coefficient * second.1);
        let y_coefficient = determinant * (- self.x_coefficient * third.0 + self.y_coefficient * second.0);

        FirstDegreePolynomial2D {
            x_coefficient,
            y_coefficient,
            independent_term: self.independent_term,
        }
    }

    /// One of three basis functions on unit triangle {(0,0),(1,0),(0,1)}
    pub fn psi_1() -> FirstDegreePolynomial2D {
        FirstDegreePolynomial2D {
            x_coefficient: -1_f64,
            y_coefficient: -1_f64,
            independent_term: 1_f64,
        }
    }

    /// One of three basis functions on unit triangle {(0,0),(1,0),(0,1)}
    pub fn psi_2() -> FirstDegreePolynomial2D {
        FirstDegreePolynomial2D {
            x_coefficient: 1_f64,
            y_coefficient: 0_f64,
            independent_term: 0_f64,
        }
    }

    /// One of three basis functions on unit triangle {(0,0),(1,0),(0,1)}
    pub fn psi_3() -> FirstDegreePolynomial2D {
        FirstDegreePolynomial2D {
            x_coefficient: 0_f64,
            y_coefficient: 1_f64,
            independent_term: 0_f64
        }
    }
}

impl Function2D for FirstDegreePolynomial2D {
    fn evaluate(&self, x: f64, y: f64) -> f64 {
        self.x_coefficient * x + self.y_coefficient * y + self.independent_term
    }
}