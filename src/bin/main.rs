use dzahui::{DzahuiWindowBuilder, DzahuiWindow};

fn main() {
    // Creating window with predetermined configuration
    let window_builder: DzahuiWindowBuilder<&str,&str,&str,&str, &str, &str> = DzahuiWindow::builder("/home/Arthur/Tesis/Dzahui/assets/untitled.obj");
    let window= window_builder.build();
    window.run();
}