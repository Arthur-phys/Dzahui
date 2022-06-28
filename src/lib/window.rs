use glutin::{event_loop::{EventLoop},window::WindowBuilder,dpi::PhysicalSize,ContextBuilder,GlRequest, Api, GlProfile, ContextWrapper, PossiblyCurrent, window::Window};
use std::{time::Instant};
use crate::shader::Shader;
use gl;

// Hold everything for window simulation
pub struct DzahuiWindow {
    gl_version: (u8,u8), // OpenGL version (normally 3.3)
    pub context: ContextWrapper<PossiblyCurrent,Window>, // Only one instance should be active at once
    pub timer: Instant,  // Timer for window
    pub shader: Shader, // Indicates the vertex and fragment shaders to compile and run
}

impl DzahuiWindow {
    // Create new instance of window
    pub fn new(height: i32, width: i32, gl_version: (u8,u8), event_loop: &EventLoop<()>, vertex_shader: &str, fragment_shader: &str) -> Self {
        // WindowBuilder with predetermined settings
        let window_builder = WindowBuilder::new().
            with_title("Dzahui").
            with_inner_size(PhysicalSize {height, width}).
            with_resizable(true);

        // OpenGL version
        let opengl_version = GlRequest::Specific(Api::OpenGl, gl_version);

        // Creating context to use in application
        let context = ContextBuilder::new().
        with_gl(opengl_version).
        with_gl_profile(GlProfile::Core).
        build_windowed(window_builder, event_loop).
        unwrap(); // core GL

        // Future compatible functions. Not backwards compatible (no previous versions of openGL)
        // The latest instance becomes the current context always
        let context = unsafe { 
        context.make_current().unwrap() 
        };

        // Loading of OpenGL functions
        gl::load_with(&|s: &str| {context.get_proc_address(s)});
        // GL Viewport
        unsafe { gl::Viewport(0,0,width,height) }


        // Use shaders chosen
        let shader = Shader::new(vertex_shader,fragment_shader);
        shader.use_shader();

        // Start clock for delta time
        let timer = Instant::now();

        DzahuiWindow { context, gl_version, timer, shader}
    }

    // Grab cursor if needed to stop it from going outside the window
    pub fn grab_cursor(&self, grab: bool) {
        self.context.window().set_cursor_grab(grab).unwrap();
    }

    // Restart timer if needed
    pub fn restart_timer(&mut self) {
        self.timer = Instant::now();
    }
}
