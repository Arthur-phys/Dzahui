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
    Cone,
    Drawable,
    CameraBuilder, drawable::mesh::MeshBuilder};

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
    context: ContextWrapper<PossiblyCurrent,Window>,
    camera: Camera,
    mesh: Box<dyn HighlightableVertices>,
    event_loop: Option<EventLoop<()>>,
    pub(crate) geometry_shader: Shader,
    pub(crate) text_shader: Shader,
    pub(crate) height: u32,
    pub(crate) width: u32,
    pub timer: Instant
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
pub struct DzahuiWindowBuilder<A, B, C, D, E, F>
    where A: AsRef<str>,
          B: AsRef<str>,
          C: AsRef<str>,
          D: AsRef<str>,
          E: AsRef<str>,
          F: AsRef<str>,
    {
    geometry_vertex_shader: Option<A>,
    geometry_fragment_shader: Option<B>,
    text_vertex_shader: Option<C>,
    text_fragment_shader: Option<D>,
    camera: CameraBuilder,
    mesh: MeshBuilder<E,F>,
    opengl_version: Option<(u8,u8)>,
    height: Option<u32>,
    width: Option<u32>
}

impl<A,B,C,D,E,F> DzahuiWindowBuilder<A,B,C,D,E,F>
    where A: AsRef<str>,
          B: AsRef<str>,
          C: AsRef<str>,
          D: AsRef<str>,
          E: AsRef<str>,
          F: AsRef<str>,
    {
    /// Creates default instance.
    fn new() -> Self {
        Self {
            geometry_vertex_shader: None,
            geometry_fragment_shader: None,
            text_vertex_shader: None,
            text_fragment_shader: None,
            opengl_version: Some((3,3)),
            camera: Camera::builder(),
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
    /// Changes opengl version.
    pub fn with_opengl_version(self,opengl_version: (u8,u8)) -> Self {
        Self {
            opengl_version: Some(opengl_version),
            ..self
        }
    }
    // Shortcut to CameraBuilder methods
    /// Changes distance (radius) to object centered
    pub fn change_distance_from_camera_to_object(self, radius: f32) -> Self {
        Self {
            camera: self.camera.change_distance_to_object(radius),
            ..self
        }
    }
    /// Changes object being targeted
    pub fn camera_with_target(self, x: f32, y: f32, z: f32) -> Self {
        Self {
            camera: self.camera.with_target(x, y, z),
            ..self
        }
    }
    /// Changes camera position in a sphere with center `camera_target`
    pub fn with_camera_position(self, theta: f32, phi: f32) -> Self {
        Self {
            camera: self.camera.with_camera_position(theta, phi),
            ..self
        }
    }
    /// Changes fov when using projection matrix
    pub fn with_fov(self, fov: f32) -> Self {
        Self {
            camera: self.camera.with_fov(fov),
            ..self
        }
    }
    /// Changes camera speed (when implemented will move things arround)
    pub fn with_speed(self, speed: f32) -> Self {
        Self {
            camera: self.camera.with_speed(speed),
            ..self
        }
    }
    /// Changes camera movement arround object being targeted
    pub fn with_sensitivity(self, sensitivity: f32) -> Self {
        Self {
            camera: self.camera.with_sensitivity(sensitivity),
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
    /// Event loop is generated and, alongside window, made current context.
    /// OpenGL functions are made available and viewport for OpenGL is set.
    /// Geometry and Text shaders are created.
    /// A new instance of Mesh2D or Mesh3D is placed inside a Box for later use.
    /// A new camera is created based on mesh (unless overriden).
    /// A timer is created.
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

        // Creating mesh placed in box to accept both Mesh2D and Mesh3D
        let mesh: Box<dyn HighlightableVertices> = match mesh {
            MeshDimension::Two(path) => Box::new(Mesh2D::new(path.as_ref())),
            MeshDimension::Three(path) => Box::new(Mesh3D::new(path.as_ref()))
        };

        let camera = self.camera.build(&mesh, self.height.unwrap(), self.width.unwrap());

        // Start clock for delta time
        let timer = Instant::now();

        DzahuiWindow {
            context,
            timer,
            geometry_shader,
            text_shader,
            mesh,
            camera,
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
    pub fn run(mut self) {

        // Obtaining Event Loop is necessary since `event_loop.run()` consumes it alongside window if let inside struct instance.
        let event_loop = Option::take(&mut self.event_loop).unwrap();

        self.mesh.send_to_gpu();

        // Create highlightable vertices
        let ui_vertices = self.mesh.create_highlightable_vertices(0.06, "./assets/sphere.obj");
        ui_vertices.send_to_gpu();

        // COPYING LITERALLY EVERYTHING FROM MAIN. REFACTOR LATER
        // Use geometry shader
        self.geometry_shader.use_shader();
        // translation for mesh to always be near (0,0)
        self.geometry_shader.set_mat4("model", self.mesh.get_model_matrix());

        // ray casting cone
        let mut cone_sphere_selector = Cone::new(Point3::new(0.0,0.0,0.0),Vector3::new(0.0,0.0,1.0),0.1);

        self.geometry_shader.set_mat4("view", &self.camera.view_matrix);
        self.geometry_shader.set_mat4("projection", &self.camera.projection_matrix);

        event_loop.run(move |event, _, control_flow| {

            match event {
                
                Event::LoopDestroyed => (), // subscribing to events occurs here
                Event::WindowEvent {event, ..} => match event {
                    
                    WindowEvent::Resized(physical_size) => self.context.resize(physical_size),
                    
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
    
                    // When cursor is moved, create new cone to select objects
                    WindowEvent::CursorMoved { device_id, position, .. } => {
                        cone_sphere_selector = Cone::from_mouse_position(1.0, Point2::new(position.x,position.y), &self.camera, &self);
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
                                            self.camera.active_view_change = true;
                                        },
                                        ElementState::Released => {
                                            self.camera.active_view_change = false;
                                        }
                                    }
                                },
                                1 => {
                                    if let ElementState::Pressed = state {
                                        let selected_sphere = cone_sphere_selector.obtain_nearest_intersection(&ui_vertices.spheres, &self.camera);
                                        println!("{:?}",selected_sphere);
                                    }
                                }
                                _ => {}
                            }
                        },
    
                        // to move camera
                        DeviceEvent::MouseMotion { delta: (x, y) } => {
                            if self.camera.active_view_change {
                                let x_offset = (x as f32) * self.camera.camera_sensitivity;
                                let y_offset = (y as f32) * self.camera.camera_sensitivity;
                                self.camera.theta -= y_offset;
                                self.camera.phi -= x_offset;
                                
                                // Do not allow 0 (or 180) degree angle (coincides with y-axis)
                                if self.camera.theta < 1.0 {
                                    self.camera.theta = 1.0;
                                } else if self.camera.theta > 179.0 {
                                    self.camera.theta = 179.0;
                                }
        
                                // update position
                                self.camera.camera_position = Point3::new(self.camera.theta.to_radians().sin()*self.camera.phi.to_radians().sin(),
                                self.camera.theta.to_radians().cos(),self.camera.theta.to_radians().sin()*self.camera.phi.to_radians().cos()) * self.camera.radius;
                                
                                // generate new matrix
                                self.camera.modify_view_matrix();
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
                self.camera.position_camera(&self);
                // Draw triangles via ebo (indices)
                self.mesh.draw(&self);
            }
            // Need to change old and new buffer to redraw
            self.context.swap_buffers().unwrap();
        })
        
    }
}
