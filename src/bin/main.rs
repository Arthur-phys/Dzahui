use dzahui::{DzahuiWindow, DzahuiWindowBuilder};

fn main() {
    dzahui::logger::spawn(log::LevelFilter::Info, "dzahui").unwrap();
    // Creating window with predetermined configuration
    /*
    let diffusion_params = DiffusionParams1D::time_dependent()
        .mu(1.0)
        .b(1.0)
        .boundary_conditions(1_f64, 15_f64)
        .initial_conditions(vec![0_f64,135_f64,1215_f64,15432_f64,212141_f64,43431_f64, 6565_f64,655_f64]);
    */

    let window_builder: DzahuiWindowBuilder = DzahuiWindow::builder("./assets/1dbar.obj")
        .solve_1d_time_dependant_diffussion([1.0, 15.0],
            [0.0, 135.0, 1215.0, 15432.0, 212141.0, 43431.0,  6565.0, 655.0], 1.0, 10.0)
        .with_integration_iteration(150);
    let window = window_builder.build();
    window.run();
}
