use ndarray::Array1;

#[derive(Debug)]
pub(crate) enum Condition {
    Dirichlet(Array1<f64>),
    Newmann(Array1<f64>),
}

#[derive(Debug)]
pub(crate) enum VertexType {
    Boundary(Condition),
    Internal(Array1<f64>)
}
