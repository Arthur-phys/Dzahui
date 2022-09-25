use ndarray::Array1;

/// # General Information
/// 
/// A boundary vertex can have one of two types of conditions over the function the equation is solved for:
/// * Dirichlet: A condition over the function at the boundary.
/// * Neumann: A condition over the normal derivative of the function at the boundary.
/// While mixed conditions can occur, The vertices in this simulator can only use one or the other.
/// 
/// # Arms
/// 
/// * Dirichlet: Holds a boundary condition which does not need pre-processing to be used.
/// * Neumann: Holds a boundary condition that most likely needs to be used to first approximate a function value.
/// 
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub(crate) enum Condition {
    Dirichlet(Array1<f64>),
    Neumann(Array1<f64>),
}

/// # General Information
/// 
/// A vertex can be of two kinds:
/// * A boundary vertex: This vertex establishes conditions for a given PDE/ODE to have a unique solution.
/// * An internal vertex: This vertex can have an initial state which will change when time starts to advance.
/// 
/// # Arms
/// 
/// * `Boundary`: Can be of two kinds: Dirichlet or Neumann.
/// * `Internal`: Holds an initial value.
#[derive(Debug,Clone)]
pub(crate) enum VertexType {
    Boundary(Condition),
    Internal(Array1<f64>)
}
