pub trait Function1D {
    fn evaluate(&self, x: f64) -> f64;
}

pub trait Differentiable1D<T> 
    where T: Function1D {
    fn differentiate(&self) -> T;
}

pub trait Composable1D<T, U> 
    where T: Function1D, U: Function1D {
    fn compose(&self, other: T) -> U;
}

pub struct FirstDegreePolynomial {
    pub(crate) coefficient: f64,
    pub(crate) independent_term: f64
}

pub struct SecondDegreePolynomial {
    quadratic_coefficient: f64,
    linear_coefficient: f64,
    independent_term: f64,
}

impl FirstDegreePolynomial {

    pub fn new(coefficient: f64, independent_term: f64) -> FirstDegreePolynomial {
        FirstDegreePolynomial {
            coefficient,
            independent_term
        }
    }

    /// Zero function factory
    pub fn zero() -> FirstDegreePolynomial {
        Self {
            coefficient: 0_f64,
            independent_term: 0_f64,
        }
    }

    /// Constant function factory
    pub fn constant(independent_term: f64) -> Self {
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
    
}

impl Function1D for FirstDegreePolynomial {

    fn evaluate(&self, x: f64) -> f64 {
        self.coefficient * x + self.independent_term
    }

}

// self(other(x))
impl Composable1D<FirstDegreePolynomial, FirstDegreePolynomial> for FirstDegreePolynomial {

    fn compose(&self, other: FirstDegreePolynomial) -> FirstDegreePolynomial {

        FirstDegreePolynomial {
            coefficient: self.coefficient * other.coefficient,
            independent_term: self.coefficient * other.independent_term + self.independent_term,
        }

    }
}

impl Differentiable1D<FirstDegreePolynomial> for FirstDegreePolynomial {

    fn differentiate(&self) -> FirstDegreePolynomial {
        FirstDegreePolynomial {
            coefficient: 0_f64,
            independent_term: self.coefficient
        }
    }

}

impl SecondDegreePolynomial {

    pub fn new(quadratic_coefficient: f64, linear_coefficient: f64, independent_term: f64) -> SecondDegreePolynomial {
        SecondDegreePolynomial {
            quadratic_coefficient,
            linear_coefficient,
            independent_term
        }
    }

}

impl Function1D for SecondDegreePolynomial {

    fn evaluate(&self, x: f64) -> f64 {
        self.quadratic_coefficient * x * x + self.linear_coefficient * x + self.independent_term
    }

}

impl Differentiable1D<FirstDegreePolynomial> for SecondDegreePolynomial {

    fn differentiate(&self) -> FirstDegreePolynomial {
        FirstDegreePolynomial {
            coefficient: 2_f64 * self.quadratic_coefficient,
            independent_term: self.linear_coefficient
        }
    }
    
}

// self(other(x))
impl Composable1D<FirstDegreePolynomial,SecondDegreePolynomial> for SecondDegreePolynomial {

    fn compose(&self, other: FirstDegreePolynomial) -> SecondDegreePolynomial {

        SecondDegreePolynomial {
            quadratic_coefficient: self.quadratic_coefficient * other.coefficient.powf(2_f64),
            linear_coefficient: 2_f64 * self.quadratic_coefficient * other.coefficient * other.independent_term + self.linear_coefficient * other.coefficient,
            independent_term: other.independent_term.powf(2_f64) + other.independent_term * self.linear_coefficient + self.independent_term
        }

    }
}
