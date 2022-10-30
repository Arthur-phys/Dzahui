pub mod single_variable;
pub mod two_variables;

// /// Empty trait to implement `PiecewiseFirstDegreePolynomial` for different amounts of partitions of an interval
// pub(crate) trait IntervalStep {}
// /// Empty trait to implement `PiecewiseFirstDegreePolynomial` for different amounts arguments. Thightly related to `IntervalStep`. Possible deprecation in the
// /// future.
// pub(crate) trait NumberOfArguments {}

// impl IntervalStep for [f64; 3] {}
// impl NumberOfArguments for [f64; 4] {}

// #[derive(Debug)]
// /// # General Information
// ///
// /// A first-degree polynomial defined in pieces. Every division of an interval, represented by `IntervalStep` has different coefficients,
// /// represented by `NumberOfArguments`. An if - else evaluation is done over the interval to decide how to evaluate the input. It may not be necessary in the
// /// future, therefore it may be deprecated.
// ///
// /// # Fields
// ///
// /// * `coefficients` - All coefficients ordered in an array that implements `NumberOfArguments`
// /// * `independent_terms` - All independent terms ordered in an array that implements `NumberOfArguments`
// /// * `interval` - An array of numbers to sepparate the real numbers implementing `IntervalStep`
// ///
// pub(crate) struct PiecewiseFirstDegreePolynomial<A: IntervalStep, B: NumberOfArguments> {
//     coefficients: B,
//     independent_terms: B,
//     interval: A,
// }

// impl PiecewiseFirstDegreePolynomial<[f64; 3], [f64; 4]> {
//     /// To create a new, raw, instance of a piecewise first-degree polynomial
//     fn new(coefficients: [f64; 4], independent_terms: [f64; 4], interval: [f64; 3]) -> Self {
//         Self {
//             coefficients,
//             independent_terms,
//             interval,
//         }
//     }

//     /// Per-interval constant piecewise first-degree polynomial
//     fn constants(independent_terms: [f64; 4], interval: [f64; 3]) -> Self {
//         Self {
//             coefficients: [0_f64; 4],
//             independent_terms,
//             interval,
//         }
//     }

//     /// Create a piecewise first-degree polynomial from a series of polynomials
//     fn from_polynomials(functions: [&FirstDegreePolynomial; 4], interval: [f64; 3]) -> Self {
//         Self {
//             coefficients: [
//                 functions[0].coefficient,
//                 functions[1].coefficient,
//                 functions[2].coefficient,
//                 functions[3].coefficient,
//             ],
//             independent_terms: [
//                 functions[0].independent_term,
//                 functions[1].independent_term,
//                 functions[2].independent_term,
//                 functions[3].independent_term,
//             ],
//             interval,
//         }
//     }
// }

// impl Function for PiecewiseFirstDegreePolynomial<[f64; 3], [f64; 4]> {
//     fn evaluate(&self, x: f64) -> f64 {
//         if x < self.interval[0] {
//             self.coefficients[0] * x + self.independent_terms[0]
//         } else if x >= self.interval[0] && x < self.interval[1] {
//             self.coefficients[1] * x + self.independent_terms[1]
//         } else if x >= self.interval[1] && x < self.interval[2] {
//             self.coefficients[2] * x + self.independent_terms[2]
//         } else {
//             self.coefficients[3] * x + self.independent_terms[3]
//         }
//     }
// }

// impl PartialEq for PiecewiseFirstDegreePolynomial<[f64; 3], [f64; 4]> {
//     fn eq(&self, other: &Self) -> bool {
//         self.coefficients == other.coefficients
//             && self.independent_terms == other.independent_terms
//             && self.interval == other.interval
//     }
// }

// impl Eq for PiecewiseFirstDegreePolynomial<[f64; 3], [f64; 4]> {}

// impl Differentiable for PiecewiseFirstDegreePolynomial<[f64; 3], [f64; 4]> {
//     fn differentiate(&self) -> Box<dyn Function> {
//         Box::new(PiecewiseFirstDegreePolynomial::constants(
//             self.coefficients,
//             self.interval,
//         ))
//     }
// }
