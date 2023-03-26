use dzahui::{DzahuiWindow, DzahuiWindowBuilder, StokesParams};

/// Simple static pressure example
fn main() {

    let naviers_params = StokesParams::static_pressure().hydrostatic_pressure(100_f64).density(1_f64).force_function(
        Box::new(|_| -10_f64)
    ).build();

    let window_builder: DzahuiWindowBuilder = DzahuiWindow::builder("./assets/1dbar.obj")
        .solve_static_pressure(naviers_params).with_integration_iteration(350);

    let window = window_builder.build();
    window.run();

}