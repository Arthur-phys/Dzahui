use dzahui::{DzahuiWindowBuilder, DzahuiWindow};
use dzahui::solvers::Solver;

fn main() {
    // Creating window with predetermined configuration
    let window_builder: DzahuiWindowBuilder = DzahuiWindow::builder("/home/Arthur/Tesis/Dzahui/assets/1dbar.obj", 
    Solver::DiffussionSolver).with_mesh_in_1d();
    let window= window_builder.build();
    window.run();
}