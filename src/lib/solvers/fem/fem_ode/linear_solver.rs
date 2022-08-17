trait LinearSolver {
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