/// # General Information
/// 
/// Represents a solver of a linear problem (master problem if you will) of the form **Ax=b** in which **A** is a square matrix, **b** 
/// is a known vector and **x** is to be found.
/// It needs to implement a solver function that returns the desired result.
/// 
trait LinearSolver {
    
    /// # General Information
    /// 
    /// A function that solves a problem with te information present in each struct implementing this trait.
    /// 
    /// # Parameters
    /// 
    /// 
    /// 
    fn solve_mat() -> Vec<f32>;
}

struct CholeskySolver {}
struct ThomasSolver {}
struct JacobiSolver {}
struct GaussSeidelSolver {}

impl LinearSolver for CholeskySolver {
    fn solve_mat() -> Vec<f32> {
        todo!()
    }
}
impl LinearSolver for ThomasSolver {
    fn solve_mat() -> Vec<f32> {
        todo!()
    }
}
impl LinearSolver for JacobiSolver {
    fn solve_mat() -> Vec<f32> {
        todo!()
    }
}
impl LinearSolver for GaussSeidelSolver {
    fn solve_mat() -> Vec<f32> {
        todo!()
    }
}