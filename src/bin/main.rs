use dzahui::{DzahuiWindowBuilder, MeshDimension, DzahuiWindow};

fn main() {
    // Creating window with predetermined configuration
    let window_builder: DzahuiWindowBuilder<&str,&str,&str,&str> = DzahuiWindow::builder();
    let window= window_builder.build(MeshDimension::Two("/home/Arthur/Tesis/Dzahui/assets/untitled.obj"));
    window.run();
}