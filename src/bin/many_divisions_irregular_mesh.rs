use dzahui::{DzahuiWindow, DzahuiWindowBuilder, DiffussionParams};

/// Simple time-independent diffussion example with an irregular mesh with many divisions.
fn main() {
    /* TIME INDEPENDENT DIFFUSSION */
    let diffussion_params = DiffussionParams::time_independent()
        .b(1.0)
        .mu(1.0)
        .boundary_conditions(1.0,15.0)
        .build();
    
    let window_builder: DzahuiWindowBuilder = DzahuiWindow::builder("./assets/1dbar_many_divisions_irregular.obj")
        .solve_1d_diffussion(diffussion_params).with_integration_iteration(150).enable_height_multiplier(50_f64);

    let window = window_builder.build();
    window.run();
    
}