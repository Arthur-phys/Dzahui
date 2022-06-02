use piston_window::*;
use Dzahui::EulerSolver;

fn main() {
    const GREEN: [f32;4] = [0.1, 0.73, 0.09, 1.0]; // Green color (values must be between 0 and 1. Divide R,G,B by 255)
    const MULTIPLIER: f64 = 5.0;
    // Initial conditions and functions
    
    let mut time: f64 = 0.0; // s
    let mut pos_x: f64 = 10.0; // m
    let mut pos_y: f64 = 500.0; // m
    let mut vel_y: f64 = -100.0; // m/s
    let x_solver = EulerSolver::new(|_init_vals: &[f64;2]| 20.0);
    let y_solver = EulerSolver::new(|_init_vals: &[f64;3]| 9.81);
    
    // Creation of window with: title, size, graphics api
    
    let opengl = OpenGL::V3_2; // OpenGL version used. Convertible to struct Api through function
    let mut window: PistonWindow = WindowSettings::new("Dzahui", [520; 2])// Generates a window of size [height=520,lenght=520]
        .exit_on_esc(true)
        .graphics_api(opengl) // Other options are: Vulkan, directX and Metal (current is OpenGL).
        .build()
        .unwrap();

    // Event loop
    while let Some(e) = window.next() { // As long as there is an event e: Event

        // Drawing of components at every update
        window.draw_2d(&e, |c, g, _| {
            // e.render_args(); from here viewport and others can be obtained
            clear([1.0; 4], g); // Clears screen with color [R,G,B,A] given a graphics implementation
            circle_arc(GREEN,10.0, 0.0, f64::_360(),[pos_x,pos_y,10.0,10.0], c.transform, g); // Draws a circle arc over a rectangle drawn like [x,y,width,height]
            
        });

        // Update values of position based on dt
        if let Some(args) = e.update_args() { // From here dt can be obtained

            [pos_x,_] = x_solver.do_step([pos_x,time],args.dt*MULTIPLIER);
            [vel_y,pos_y,time] = y_solver.do_step([vel_y,pos_y,time],args.dt*MULTIPLIER);
        }
    }
}