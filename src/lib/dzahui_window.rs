use gl;
use std::time::Instant;
use cgmath::{
    Point3,
    Vector3,
    Point2};
use glutin::{
    event_loop::{EventLoop,ControlFlow},
    window::WindowBuilder,
    dpi::PhysicalSize,
    ContextBuilder,
    GlRequest,
    Api,
    GlProfile,
    ContextWrapper,
    PossiblyCurrent,
    window::Window,
    event::{Event, WindowEvent, DeviceEvent, ElementState}};
use crate::{
    shader::Shader,
    MeshDimension,
    Mesh2D,
    Mesh3D,
    HighlightableVertices,
    Camera,
    Cone, Drawable};

/// # General Information
/// 
/// DzahuiWindow holds every important component to create an instancec of a simulator.
/// 
/// # Fields
/// 
/// * `context` - Holds an *instance* of OpenGL. This normally means that all configuration associated with rendering is stored here. Only one context is allowed.
/// * `timer` - Gives current time since creation of window. Call with `timer.elapsed()`.
/// * `geometry_shader` - Geometry_shaders to compile and use. Responsible for mesh drawing.
///  * `text_shader` - Text shaders to compile and use. Responsible form text rendering.
/// * `height` - Height of window created.
/// * `width` - Width of window created.
/// 
pub struct DzahuiWindow {
    /// Only one instance should be active at once
    pub context: ContextWrapper<PossiblyCurrent,Window>,
    pub timer: Instant,
    pub geometry_shader: Shader,
    text_shader: Shader,
    pub(crate) height: u32,
    pub(crate) width: u32,
    event_loop: Option<EventLoop<()>>
}

/// # General Information
/// 
/// The window builder. When using function `builder` in **DzahuiWindow** without parameters a sensible default is obtained.
/// 
/// # Fields
/// 
/// * `geometry_shader` - Shader used to render triangulated 3D and 2D meshes. Defaults to assets/vertex_shader.vs and assets/fragment_shader.fs.
/// * `text_shader` - Shader used to render text. Defaults to assets/text_vertex_shader.vs and assets/text_fragment_shader.fs.
/// * `height` - Height of window. Defaults to 600 px.
/// * `width` - Width of window. Defaults to 800 px.
/// 
#[derive(Default, Debug)]
pub struct DzahuiWindowBuilder<A, B, C, D>
    where A: AsRef<str>,
          B: AsRef<str>,
          C: AsRef<str>,
          D: AsRef<str>
    {
    geometry_vertex_shader: Option<A>,
    geometry_fragment_shader: Option<B>,
    text_vertex_shader: Option<C>,
    text_fragment_shader: Option<D>,
    opengl_version: Option<(u8,u8)>,
    height: Option<u32>,
    width: Option<u32>
}

impl<A,B,C,D> DzahuiWindowBuilder<A,B,C,D>
    where A: AsRef<str>,
          B: AsRef<str>,
          C: AsRef<str>,
          D: AsRef<str>
    {
    /// Creates default instance.
    fn new() -> Self {
        Self {
            geometry_vertex_shader: None,
            geometry_fragment_shader: None,
            text_vertex_shader: None,
            text_fragment_shader: None,
            opengl_version: Some((3,3)),
            height: Some(600),
            width: Some(800)
        }
    }
    /// Changes geometry shader.
    pub fn with_geometry_shader(self, vertex_shader: A, fragment_shader: B) -> Self {
        Self {
            geometry_vertex_shader: Some(vertex_shader),
            geometry_fragment_shader: Some(fragment_shader),
            ..self
        }
    }
    /// Changes text shader.
    pub fn with_text_shader(self, vertex_shader: C, fragment_shader: D) -> Self {
        Self {
            text_vertex_shader: Some(vertex_shader),
            text_fragment_shader: Some(fragment_shader),
            ..self
        }
    }
    /// Changes height and width.
    pub fn with_height_and_width(self, height: u32, width: u32) -> Self {
        Self {
            height: Some(height),
            width: Some(width),
            ..self
        }
    }
    // Changes opengl version.
    pub fn with_opengl_version(self,opengl_version: (u8,u8)) -> Self {
        Self {
            opengl_version: Some(opengl_version),
            ..self
        }
    }
    /// # General Information
    /// 
    /// Builds DzahuiWindow from parameters given or sensible defaults.
    /// 
    /// # Details
    /// 
    /// First it generates a window builder with title 'Dzahui', size according to builder and always resizable.
    /// Then and OpenGL version is assigned based on builder.
    /// Event loop is generated and made current context alongside window.
    /// OpenGL functions are made available and viewport por OpenGL is set.
    /// Geometry and Text shaders are created.
    /// An instance of DzahuiWindow is created.
    /// 
    /// 
    /// # Parameters
    /// 
    /// * `self` - All configuration required is within self. Default shaders are hardcoded in here.
    /// 
    pub fn build(self) -> DzahuiWindow {
        let window_builder = WindowBuilder::new().
            with_title("Dzahui").
            with_inner_size(PhysicalSize {height: self.height.unwrap(), width: self.width.unwrap()}).
            with_resizable(true);
        
        let opengl_version = GlRequest::Specific(Api::OpenGl, self.opengl_version.unwrap());

        // Generating event_loop to be used
        let event_loop = EventLoop::new();

        // Creating context to use in application
        let context = ContextBuilder::new().
        with_gl(opengl_version).
        // core GL profile
        // Future compatible functions. Not backwards compatible (no previous versions of openGL).
        with_gl_profile(GlProfile::Core).
        build_windowed(window_builder, &event_loop).
        unwrap();

        // The latest instance becomes the current context always
        let context = unsafe { 
            context.make_current().unwrap() 
        };

        // Loading OpenGL functions. Only done once
        gl::load_with(&|s: &str| {context.get_proc_address(s)});
        // GL Viewport
        unsafe { 
            gl::Viewport(0,0,self.width.unwrap() as i32,self.height.unwrap() as i32);
        }

        // Use text_shaders chosen
        let vertex_shader: String = if let Some(vertex_shader) = self.text_vertex_shader {
            vertex_shader.as_ref().to_string()
        } else {"./assets/text_vertex_shader.vs".to_string()};
        
        let fragment_shader: String = if let Some(fragment_shader) = self.text_fragment_shader {
            fragment_shader.as_ref().to_string()
        } else {"./assets/text_fragment_shader.fs".to_string()};
        
        let text_shader = Shader::new(vertex_shader,fragment_shader);
        
        // Use geometry_shaders chosen
        let vertex_shader: String = if let Some(vertex_shader) = self.geometry_vertex_shader {
            vertex_shader.as_ref().to_string()
            } else {"./assets/geometry_vertex_shader.vs".to_string()};

        let fragment_shader: String = if let Some(fragment_shader) = self.geometry_fragment_shader {
            fragment_shader.as_ref().to_string()
            } else {"./assets/geometry_fragment_shader.fs".to_string()};
        
        let geometry_shader = Shader::new(vertex_shader,fragment_shader);

        // Start clock for delta time
        let timer = Instant::now();

        DzahuiWindow {
            context,
            timer,
            geometry_shader,
            text_shader,
            event_loop: Some(event_loop),
            height: self.height.unwrap(),
            width: self.width.unwrap()
        }
    }
}

impl DzahuiWindow {

    pub fn builder<A,B,C,D>() -> DzahuiWindowBuilder<A,B,C,D> where 
    A: AsRef<str>,
    B: AsRef<str>,
    C: AsRef<str>,
    D: AsRef<str> {
        DzahuiWindowBuilder::new()
    }

    /// # General Information
    /// 
    /// Grab cursor if needed to stop it from going outside the window.
    ///
    /// # Parameters
    /// 
    /// * `&self` - To use current context and grab cursor
    /// * `grab` - A boolean value to change the state of the grab variable inside context.
    /// 
    pub fn grab_cursor(&self, grab: bool) {
        self.context.window().set_cursor_grab(grab).unwrap();
    }

    /// # General information
    /// 
    /// To restart timer of window in case is needed.
    /// 
    /// # Parameters
    /// 
    /// `&mut self` - To change status of field `timer`.
    /// 
    pub fn restart_timer(&mut self) {
        self.timer = Instant::now();
    }

    /// # General Information
    /// 
    /// Run window with a mesh and an event loop. Consumes every object since they're not to be used afters.
    /// 
    /// # Parameters
    /// 
    /// * `self` - A window instance.
    /// * `mesh` - A file to draw a mesh from.
    /// 
    pub fn run<A: AsRef<str>>(mut self, mesh: MeshDimension<A>) {

        // Obtaining Event Loop is necessary since `event_loop.run()` consumes it alongside window if let inside struct instance.
        let event_loop = Option::take(&mut self.event_loop).unwrap();

        // Creating mesh placed in box to accept both Mesh2D and Mesh3D
        let mesh: Box<dyn HighlightableVertices> = match mesh {
            MeshDimension::Two(path) => Box::new(Mesh2D::new(path.as_ref())),
            MeshDimension::Three(path) => Box::new(Mesh3D::new(path.as_ref()))
        };
        mesh.send_to_gpu();

        // Create highlightable vertices
        let ui_vertices = mesh.create_highlightable_vertices(1.0, "./assets/sphere.obj");
        ui_vertices.send_to_gpu();

        // COPYING LITERALLY EVERYTHING FROM MAIN. REFACTOR LATER
        // Use geometry shader
        self.geometry_shader.use_shader();
        // translation for mesh to always be near (0,0)
        self.geometry_shader.set_mat4("model", mesh.get_model_matrix());

        // Camera creation
        let mut camera = Camera::new(&mesh, 600.0, 800.0);

        // ray casting cone
        let mut cone_sphere_selector = Cone::new(Point3::new(0.0,0.0,0.0),Vector3::new(0.0,0.0,1.0),0.1);

        self.geometry_shader.set_mat4("view", &camera.view_matrix);
        self.geometry_shader.set_mat4("projection", &camera.projection_matrix);

        event_loop.run(move |event, _, control_flow| {

            match event {
                
                Event::LoopDestroyed => (), // subscribing to events occurs here
                Event::WindowEvent {event, ..} => match event {
                    
                    WindowEvent::Resized(physical_size) => self.context.resize(physical_size),
                    
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
    
                    // When cursor is moved, create new cone to select objects
                    WindowEvent::CursorMoved { device_id, position, .. } => {
                        cone_sphere_selector = Cone::from_mouse_position(1.0, Point2::new(position.x,position.y), &camera, &self);
                    },
                    
                    // Close on esc
                    WindowEvent::KeyboardInput {input, is_synthetic, ..} => {
                        if !is_synthetic && input.scancode == 1 {
                            *control_flow = ControlFlow::Exit;
                        }
                    },
    
                    _ => ()
                },
                
                Event::DeviceEvent { device_id, event } => {
                    match event {
                        // to activate moving camera
                        DeviceEvent::Button { button, state } => {
                            match button {
                                2 => {
                                    match state {
                                        ElementState::Pressed => {
                                            camera.active_view_change = true;
                                        },
                                        ElementState::Released => {
                                            camera.active_view_change = false;
                                        }
                                    }
                                },
                                1 => {
                                    if let ElementState::Pressed = state {
                                        let selected_sphere = cone_sphere_selector.obtain_nearest_intersection(&ui_vertices.spheres, &camera);
                                        println!("{:?}",selected_sphere);
                                    }
                                }
                                _ => {}
                            }
                        },
    
                        // to move camera
                        DeviceEvent::MouseMotion { delta: (x, y) } => {
                            if camera.active_view_change {
                                let x_offset = (x as f32) * camera.camera_sensitivity;
                                let y_offset = (y as f32) * camera.camera_sensitivity;
                                camera.theta -= y_offset;
                                camera.phi -= x_offset;
                                
                                // Do not allow 0 (or 180) degree angle (coincides with y-axis)
                                if camera.theta < 1.0 {
                                    camera.theta = 1.0;
                                } else if camera.theta > 179.0 {
                                    camera.theta = 179.0;
                                }
        
                                // update position
                                camera.camera_position = Point3::new(camera.theta.to_radians().sin()*camera.phi.to_radians().sin(),
                                camera.theta.to_radians().cos(),camera.theta.to_radians().sin()*camera.phi.to_radians().cos()) * camera.radius;
                                
                                // generate new matrix
                                camera.modify_view_matrix();
                            }
    
    
                        }
    
                        _ => {}
                    }
                },
    
                Event::RedrawRequested(_) => (),
    
                _ => (),
            }
            // Render
            unsafe {
                // Update to some color
                gl::ClearColor(0.33, 0.33, 0.33, 0.8);
                // Clear Screem
                gl::Clear(gl::COLOR_BUFFER_BIT);
                // Draw sphere(s)
                ui_vertices.draw(&self);
                // set camera
                camera.position_camera(&self);
                // Draw triangles via ebo (indices)
                mesh.draw(&self);
            }
            // Need to change old and new buffer to redraw
            self.context.swap_buffers().unwrap();
        })
        
    }
}
