mod camera;
mod mesh;
mod shader;
pub use self::mesh::{Mesh,Dimension};


// FIRST EXCERCISE
/// Euler's method for ordinary differential equations.
/// The form of the equation is assumed to be "y'(n) = f(t,y,y',..,y'(n-1))".
/// 
/// # Parameters
/// - initial_val: Vec<f32>
///   The previous values of every variable. The derivatives go in descending order: [y'(n),y'(n-1),...,t]
/// - step: f32
///   The step used to calculate the approximation.
/// - f: T where T: Fn(Vec<f32>) -> f32
///   Reffers to the function on the left side of the above equation.
/// 
/// # Returns
/// - Vec<f32>
///   A vector of values corresponding to the new approximation. The derivatives are delivered as in 'initial_val'
/// 
pub trait FunctionArguments: Into<Vec<f64>> + Clone + std::convert::TryFrom<Vec<f64>> {
}

impl FunctionArguments for [f64; 2]{}
impl FunctionArguments for [f64; 3]{}
impl FunctionArguments for [f64; 4]{}
impl FunctionArguments for [f64; 5]{}

pub struct EulerSolver<A, F> {
    derivative_function: F,
    phantom: std::marker::PhantomData<A>
}

impl<A: FunctionArguments, F: Fn(&A) -> f64> EulerSolver<A, F> {
    pub fn new(derivative_function: F) -> EulerSolver<A, F> {
        EulerSolver {
            derivative_function,
            phantom: std::marker::PhantomData
        }
    }

    pub fn do_step(&self, values: A, step: f64) -> A {
        
        let f_eval: f64 = (self.derivative_function)(&values);
        let as_vec: Vec<f64> = values.into();
        

        let mut next_values: Vec<f64> = vec![];
        
        let mut value: f64 = as_vec.get(0).unwrap() + step*f_eval;
        let t_new: f64 = as_vec.get(as_vec.len()-1).unwrap() + step;
        
        next_values.push(value);

        as_vec[1..as_vec.len()-1].into_iter().for_each(|x| {
            let new_val: f64 = x + step*value;
            value = new_val;
            next_values.push(new_val);
        });
        next_values.push(t_new);
        
        match A::try_from(next_values) {
            Ok(v) => v,
            Err(_) => panic!("Nooo")
        }
    }
}