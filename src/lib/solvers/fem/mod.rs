pub mod fem_ode;
pub mod function;

#[derive(Debug)]
pub enum Solver {
    DiffussionSolver([f64;2],f64,f64),
    NavierStokes1DSolver,
    NavierStokes2DSolver,
    None
}