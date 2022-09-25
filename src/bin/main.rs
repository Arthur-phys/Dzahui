use dzahui::{DzahuiWindowBuilder, DzahuiWindow};
use dzahui::solvers::Solver;

fn main() {
    // Creating window with predetermined configuration
    let window_builder: DzahuiWindowBuilder = DzahuiWindow::builder("/home/Arthur/Tesis/Dzahui/assets/sphere.obj", 
    Solver::DiffussionSolver).with_mesh_in_3d();
    let window= window_builder.build();
    window.run();
}