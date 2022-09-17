use dzahui::{DzahuiWindowBuilder, DzahuiWindow};
use dzahui::solvers::Solver;

fn main() {
    // Creating window with predetermined configuration
    let window_builder: DzahuiWindowBuilder = DzahuiWindow::builder("/home/Arthur/Tesis/Dzahui/assets/big_mesh.obj", 
    Solver::DiffussionSolver);
    let window= window_builder.build();
    window.run();
}