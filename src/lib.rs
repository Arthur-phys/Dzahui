pub use self::mesh::Mesh;

mod mesh;

// PRIMER EJERCICIO
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
pub fn euler_step<T: Fn(&Vec<f32>) -> f32>(mut initial_val: Vec<f32>, step: f32, f: T) -> Vec<f32> {
    let f_eval: f32 = f(&initial_val);
    let mut next_values: Vec<f32> = vec![];
    
    let mut value: f32 = initial_val.get(0).unwrap() + step*f_eval;
    let t_new: f32 = initial_val.get(initial_val.len()-1).unwrap() + step;
    
    next_values.push(value);

    initial_val[1..initial_val.len()-1].into_iter().for_each(|x| {
        let new_val: f32 = x + step*value;
        value = new_val;
        next_values.push(new_val);
    });
    next_values.push(t_new);

    next_values
}