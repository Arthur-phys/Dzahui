use dzahui::{DzahuiWindow, DzahuiWindowBuilder};

fn main() {
    // Creating window with predetermined configuration
    let window_builder: DzahuiWindowBuilder = DzahuiWindow::builder("./assets/1dbar.obj")
        .solve_1d_time_dependant_diffussion([1_f64, 15_f64], vec![0_f64;8], 1_f64, 1_f64)
        .with_integration_iteration(150);
    let window = window_builder.build();
    window.run();
}
