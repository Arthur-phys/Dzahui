pub mod fem_ode;
pub mod function;

// enum PolynomialDegree {
//     One,
//     Two,
//     Three,
// }

// enum Equation {
//     Diffusion,
//     NavierStokes
// }

#[derive(Debug)]
pub enum Solver {
    DiffussionSolver,
    NavierStokes1DSolver,
    NavierStokes2DSolver,
}