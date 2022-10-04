use dzahui::{DzahuiWindowBuilder, DzahuiWindow};
use dzahui::solvers::Solver;

fn main() {
    // Creating window with predetermined configuration
    let window_builder: DzahuiWindowBuilder = DzahuiWindow::builder("./assets/1dbar.obj").solve_1d_diffussion([1243_f64,100_f64], 32_f64, 15_f64).with_mesh_in_1d();
    let window= window_builder.build();
    window.run();
}