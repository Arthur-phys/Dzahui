pub mod linear_basis;


/// A trait representing a function evaluation
pub trait Function {
    fn evaluate(&self, x: f64) -> f64;
}

/// Composition of a function, when it's affine, is simple. Normal composition requires a Box<dyn Function>, which, for some reason, does not work with
/// certain functionality later on. A reformatting of this and other modules will be made to solve this issue (Maybe traits become unnecesary since Fn, FnMut and
/// FnOnce exist). 
pub trait Affine: Function {
    fn compose(&self, other: Self) -> Self;
}

/// Differentiation of a function returns another function of some kind. Every one-time differentiable function implements this.
pub trait Differentiable: Function {
    fn differentiate(&self) -> Box<dyn Function>;
}

/// # General Information
/// 
/// A first-degree polynomial is a simple function with two parameters: a coefficient and an independent term.
/// 
/// # Fields
/// 
/// * `coefficient` - That wich is multiplied by the variable.
/// * `independent_term` - That which is summed to the variable (akin to a 1D affine transformation).
/// 
pub(crate) struct FirstDegreePolynomial {
    pub(crate) coefficient: f64,
    pub(crate) independent_term: f64,
}

impl FirstDegreePolynomial {
    
    /// New instance of FirstDegreePolynomial
    fn new(coefficient: f64, independent_term: f64) -> Self {
        Self {
            coefficient,
            independent_term
        }
    }

    /// Zero function builder
    fn zero() -> Self {
        Self {
            coefficient: 0_f64,
            independent_term: 0_f64
        }
    }

    /// Constant function builder
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

/// Empty trait to implement `PiecewiseFirstDegreePolynomial` for different amounts of partitions of an interval
pub(crate) trait IntervalStep {}
/// Empty trait to implement `PiecewiseFirstDegreePolynomial` for different amounts arguments. Thightly related to `IntervalStep`. Possible deprecation in the
/// future. 
pub(crate) trait NumberOfArguments {}

impl IntervalStep for [f64;3] {}
impl NumberOfArguments for [f64;4] {}

#[derive(Debug)]
/// # General Information
/// 
/// A first-degree polynomial defined in pieces. Every division of an interval, represented by `IntervalStep` has different coefficients,
/// represented by `NumberOfArguments`. An if - else evaluation is done over the interval to decide how to evaluate the input. It may not be necessary in the
/// future, therefore it may be deprecated.
/// 
/// # Fields
/// 
/// * `coefficients` - All coefficients ordered in an array that implements `NumberOfArguments` 
/// * `independent_terms` - All independent terms ordered in an array that implements `NumberOfArguments`
/// * `interval` - An array of numbers to sepparate the real numbers implementing `IntervalStep`
/// 
pub(crate) struct PiecewiseFirstDegreePolynomial<A: IntervalStep, B: NumberOfArguments> {
    coefficients: B,
    independent_terms: B,
    interval: A
}

impl PiecewiseFirstDegreePolynomial<[f64;3],[f64;4]> {
    
    /// To create a new, raw, instance of a piecewise first-degree polynomial
    fn new(coefficients: [f64;4], independent_terms: [f64;4], interval: [f64;3]) -> Self {
        Self {
            coefficients,
            independent_terms,
            interval
        }
    }

    /// Per-interval constant piecewise first-degree polynomial 
    fn constants(independent_terms: [f64;4], interval: [f64;3]) -> Self {
        Self {
            coefficients: [0_f64;4],
            independent_terms,
            interval
        }
    }

    /// Create a piecewise first-degree polynomial from a series of polynomials
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

/// Transformation Factory to create first-degree polynomials to translate/scale other functions. May be deprecated in the future in favor of methods over the
/// original structs or expanded upon giving it mopre versatily.
pub(crate) struct TransformationFactory();

impl TransformationFactory {

    /// Transformation to 0,1 interval from any other interval.
    pub fn build(&self, beg: f64, end: f64) -> FirstDegreePolynomial {
        let coefficient = 1_f64 / (end - beg);
        let independent_term = - beg / (end - beg); 
        FirstDegreePolynomial { coefficient, independent_term }
    }

    /// Transformation to -1,1 interval from any other interval.
    pub fn build_to_m1_p1(&self, beg: f64, end: f64) -> FirstDegreePolynomial {
        let coefficient = (end - beg) / 2_f64;
        let independent_term = (end + beg) / 2_f64;
        FirstDegreePolynomial { coefficient, independent_term }
    }

}