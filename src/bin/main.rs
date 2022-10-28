use dzahui::{DzahuiWindow, DzahuiWindowBuilder};

fn main() {
    // Creating window with predetermined configuration
    let window_builder: DzahuiWindowBuilder = DzahuiWindow::builder("./assets/1dbar.obj")
        .solve_1d_diffussion([1_f64, 15_f64], 1_f64, 1_f64);
    let window = window_builder.build();
    window.run();
}
