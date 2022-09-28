pub mod fem_ode;
pub mod function;

#[derive(Debug)]
pub enum Solver {
    DiffussionSolver,
    NavierStokes1DSolver,
    NavierStokes2DSolver,
}