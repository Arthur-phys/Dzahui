use super::{Vertex1D, BoundaryVertex1D, PolynomialDegree, DiffEquationSolver};

struct DiffussionSolverBuilder {
    boundary_vertices: [BoundaryVertex1D; 2],
    vertices: Option<Vec<Vertex1D>>,
    polynomial_degree: Option<PolynomialDegree>,
}

impl DiffussionSolverBuilder {
    fn build(self) -> DiffussionSolver {
        todo!()
    }
}

struct DiffussionSolver {
    boundary_vertices: [BoundaryVertex1D; 2],
    vertices: Vec<Vertex1D>,
}

impl DiffEquationSolver for DiffussionSolver {
    fn solve() -> Vec<f32> {
        todo!()
    }
}