use piston_window::*;
use Dzahui::euler_step;

fn main() {
    const GREEN: [f32;4] = [0.1, 0.73, 0.09, 1.0]; // Green color (values must be between 0 and 1. Divide R,G,B by 255)
    // Initial conditions and functions
    let mut time: f32 = 0.0; // s
    let mut pos_x: f32 = 10.0; // m
    let mut pos_y: f32 = 500.0; // m
    let mut vel_y: f32 = -50.0; // m/s
    let f_x = |init_vals: &Vec<f32>| {40.0}; // m/s takes two args
    let f_y = |init_vals: &Vec<f32>| {9.81}; // m/s^2 takes three args coordinate system is ↓→, therefore gravity is positive

    // Creation of window with: title, size, graphics api
    let opengl = OpenGL::V3_2; // OpenGL version used. Convertible to struct Api through function
    let mut window: PistonWindow = WindowSettings::new("Dzahui", [520; 2])// Generates a window of size [height=520,lenght=520]
        .exit_on_esc(true)
        .graphics_api(opengl) // Other options are: Vulkan, directX and Metal (current is OpenGL).
        .build()
        .unwrap();

    // Event loop
    while let Some(e) = window.next() { // As long as there is an event e: Event

        // Drawing of components
        window.draw_2d(&e, |c, g, _| {
            // e.render_args(); from here viewport and others can be obtained
            clear([1.0; 4], g); // Clears screen with color [R,G,B,A] given a graphics implementation
            circle_arc(GREEN,10.0, 0.0, f64::_360(),[pos_x as f64,pos_y as f64,10.0,10.0], c.transform, g); // Draws a circle arc over a rectangle drawn like [x,y,width,height]
            
        });
        // Update values of position based on dt
        if let Some(args) = e.update_args() { // From here dt can be obtained

            if let [_pos_x,_] = &euler_step(vec![pos_x,time],args.dt as f32,f_x)[..] {
                pos_x = *_pos_x;
            }
            if let [_vel_y,_pos_y,_time] = &euler_step(vec![vel_y,pos_y,time],args.dt as f32,f_y)[..] {
                vel_y = *_vel_y;
                pos_y = *_pos_y;
                time = *_time;
            }
        }
    }
}