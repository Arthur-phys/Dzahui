use dzahui::{DzahuiWindow, DzahuiWindowBuilder, solvers::diffusion_solver::DiffussionParams};

fn main() {
    dzahui::logger::spawn(log::LevelFilter::Info, "dzahui").unwrap();

    // let diffussion_params = DiffussionParams::time_dependent()
    // .b(1.0)
    // .mu(1.0)
    // .boundary_conditions(1.0, 15.0)
    // .initial_conditions([0_f64,135_f64,1215_f64,15432_f64,212141_f64,43431_f64, 6565_f64,655_f64]);

    let diffussion_params = DiffussionParams::time_independent()
        .b(1.0)
        .mu(1.0)
        .boundary_conditions(1.0,15.0);
    
    // Creating window with predetermined configuration
    // let window_builder: DzahuiWindowBuilder = DzahuiWindow::builder("./assets/1dbar.obj")
    //     .solve_1d_time_dependent_diffussion(diffussion_params)
    //     .with_integration_iteration(150).with_time_step(0.001);

    let window_builder: DzahuiWindowBuilder = DzahuiWindow::builder("./assets/1dbar.obj")
        .solve_1d_diffussion(diffussion_params).with_integration_iteration(150);

    let window = window_builder.build();
    window.run();
}
