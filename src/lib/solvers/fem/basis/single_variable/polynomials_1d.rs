pub struct Constant(f64);

pub struct FirstDegreePolynomial {
    coefficient: f64,
    independent_term: f64
}

pub struct SecondDegreePolynomial {
    cuadratic_coefficient: f64,
    linear_coefficient: f64,
    independent_term: f64,
}

impl Constant {

    pub fn evaluate(&self, _x: f64) -> f64 {
        self.0
    }
    pub fn differentiate(&self) -> Self {
        Constant(0_f64)
    }

}

impl FirstDegreePolynomial {
    
    pub fn evaluate(&self, x: f64) -> f64 {
        self.coefficient * x + self.independent_term
    }
    pub fn differentiate(&self) -> Constant {
        Constant(self.coefficient)
    }

}

impl SecondDegreePolynomial {

    pub fn evaluate(&self, x: f64) -> f64 {
        self.cuadratic_coefficient * x * x + self.linear_coefficient * x + self.independent_term
    }
    pub fn differentiate(&self) -> FirstDegreePolynomial {
        FirstDegreePolynomial {
            coefficient: 2_f64 * self.cuadratic_coefficient,
            independent_term: self.linear_coefficient
        }
    }

}

pub fn new_constant(constant: f64) -> Constant {
    Constant(constant)
}

pub fn new_1d_linear(coefficient: f64, independent_term:  f64) -> FirstDegreePolynomial {
    FirstDegreePolynomial {
        coefficient,
        independent_term
    }
}
pub fn new_1d_cuadratic(cuadratic_coefficient: f64, linear_coefficient: f64, independent_term: f64) -> SecondDegreePolynomial {
    SecondDegreePolynomial {
        cuadratic_coefficient,
        linear_coefficient,
        independent_term
    }
}
