// Internal dependencies
use crate::solvers::basis::functions::{Function2D, Function2D2D, Composable2D, Differentiable2D};

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
/// * `xy_coefficient` - constant that multiplies xy term.
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

/// # General Information
/// 
/// Represents a matrix transformation in 2D. A normal matrix composed of vectors or arrays is not used since
/// more control over the traits that are implemented on the struct makes latter implementations easier.
/// 
/// # Fields
/// 
/// * `a` - Element [0,0] in matrix
/// * `b` - Element [0,1] in matrix
/// * `c` - Element [1,0] in matrix
/// * `d` - Element [1,1] in matrix
/// 
pub struct Transformation2D {
    a: f64,
    b: f64,
    c: f64,
    d: f64
}

impl Transformation2D {

    ///  New instance
    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Transformation2D {
        Transformation2D {
            a,
            b,
            c,
            d
        }
    }

    /// Inverse of a  2x2 matrix
    pub fn inverse(self) -> Transformation2D {
        
        let determinant = 1_f64 / (self.a * self.d - self.b * self.c);
        
        Transformation2D {
             a: self.d * determinant,
             b: - self.b * determinant,
             c: - self.c * determinant,
             d: self.a * determinant
        }
    }

}

impl Function2D2D for Transformation2D {
    fn evaluate(&self, x: f64, y: f64) -> (f64,f64) {
        (self.a * x + self.b * y, self.c * x + self.d * y)
    }
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
            x_coefficient: 0_f64,
            y_coefficient: 0_f64,
            independent_term: 0_f64,
        }
    }

    /// Constant function factory.
    pub fn constant(independent_term: f64) -> FirstDegreePolynomial2D {
        Self {
            x_coefficient: 0_f64,
            y_coefficient: 0_f64,
            independent_term,
        }
    }

    /// Translate a function by a given point
    pub fn translate(self, w: f64, z: f64) -> FirstDegreePolynomial2D {
        Self {
            x_coefficient: self.x_coefficient,
            y_coefficient: self.y_coefficient,
            independent_term: self.independent_term - self.x_coefficient * w - self.y_coefficient * z,
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

impl Composable2D<Transformation2D, FirstDegreePolynomial2D> for FirstDegreePolynomial2D {
    
    fn compose(self, other: Transformation2D) -> Result<FirstDegreePolynomial2D,crate::Error> {

        let x_coefficient = other.a * self.x_coefficient + other.c * self.y_coefficient;
        let y_coefficient = other.b * self.x_coefficient + other.d *  self.y_coefficient;

        Ok(FirstDegreePolynomial2D {
            x_coefficient,
            y_coefficient,
            independent_term: self.independent_term,
        })

    }
}

impl Differentiable2D<FirstDegreePolynomial2D,FirstDegreePolynomial2D> for FirstDegreePolynomial2D {
    
    fn differentiate_x(&self) -> Result<FirstDegreePolynomial2D,crate::Error> {
        Ok(
            FirstDegreePolynomial2D {
                x_coefficient: 0_f64,
                y_coefficient: 0_f64,
                independent_term: self.x_coefficient
            }
        )

    }

    fn differentiate_y(&self) -> Result<FirstDegreePolynomial2D,crate::Error> {
        Ok(
            FirstDegreePolynomial2D {
                x_coefficient: 0_f64,
                y_coefficient: 0_f64,
                independent_term: self.y_coefficient
            }
        )

    }
}