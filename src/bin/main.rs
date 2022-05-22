use piston_window::*;
use Dzahui::euler_step;

fn main() {
    let opengl = OpenGL::V3_2; // OpenGL version used. Convertible to struct Api through function
    let mut window: PistonWindow = WindowSettings::new("Dzahui", [520; 2])// Generates a window of size [height=520,lenght=520]
        .exit_on_esc(true)
        .graphics_api(opengl) // Other options are: Vulkan, directX and Metal (current is OpenGL).
        .build()
        .unwrap();

    while let Some(e) = window.next() { // As long as there is an event e: Event
        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g); // Clears screen with color [R,G,B,A] given a graphics implementation
            let c = c.trans(0.0,0.0);
            let red = [1.0,0.0,0.0,1.0];
            let green = [0.1, 0.73, 0.09, 1.0]; // Green ball
            circle_arc(green,10.0, 0.0, f64::_360(),[10.0,505.0,5.0,5.0], c.transform, g); // Draws a circle arc over a rectangle drawn like [x,y,width,height]
        });
    }
}