use cgmath::{Matrix4, Point2, Point3, SquareMatrix, Vector3};
use gl;
use glutin::{
    dpi::PhysicalSize,
    event::{DeviceEvent, ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
    Api, ContextBuilder, ContextWrapper, GlProfile, GlRequest, PossiblyCurrent,
};
use std::time::Instant;

use super::camera::{cone::Cone, Camera, CameraBuilder};
use super::drawable::{text::CharacterSet, Bindable, Drawable};
use super::shader::Shader;
use crate::{mesh::{mesh_builder::MeshBuilder, Mesh}, solvers::diffusion_solver::time_dependent::DiffussionSolverTimeDependent};
use crate::{
    mesh::mesh_builder::MeshDimension,
    solvers::{
        diffusion_solver::time_independent::DiffussionSolverTimeIndependent, DiffEquationSolver,
        Solver,
    },
};

/// # General Information
///
/// DzahuiWindow holds every important component to create an instancec of a simulator. Only one instance should be active at once.
///
/// # Fields
///
/// * `context` - Holds an *instance* of OpenGL. This normally means that all configuration associated with rendering is stored here. Only one context is allowed.
/// * `geometry_shader` - Geometry_shaders to compile and use. Responsible for mesh drawing.
/// * `event_loop` - To obtain user input in window.
/// * `text_shader` - Text shaders to compile and use. Responsible for text rendering.
/// * `height` - Height of window created.
/// * `width` - Width of window created.
/// * `timer` - Gives current time since creation of window. Call with `timer.elapsed()`.
/// * `camera` - Camera configuration creates view and projetion matrices, which directly tells OpenGL what to and not to render.
/// * `mesh` - A mesh to draw to screen. Represents an object tessellated into triangles/traingular prisms.
///
pub struct DzahuiWindow {
    context: ContextWrapper<PossiblyCurrent, Window>,
    pub(crate) geometry_shader: Shader,
    event_loop: Option<EventLoop<()>>,
    mouse_coordinates: Point2<f32>,
    character_set: CharacterSet,
    integration_iteration: usize,
    pub(crate) height: u32,
    pub(crate) width: u32,
    vertex_selector: Cone,
    text_shader: Shader,
    pub timer: Instant,
    camera: Camera,
    solver: Solver,
    mesh: Mesh,
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
#[derive(Debug)]
pub struct DzahuiWindowBuilder {
    geometry_fragment_shader: Option<String>,
    geometry_vertex_shader: Option<String>,
    text_fragment_shader: Option<String>,
    integration_iteration: Option<usize>,
    opengl_version: Option<(u8, u8)>,
    character_set: Option<String>,
    text_vertex_shader: Option<String>,
    vertex_selector: Option<f32>,
    mesh: MeshBuilder,
    mesh_dimension: MeshDimension,
    camera: CameraBuilder,
    height: Option<u32>,
    width: Option<u32>,
    solver: Solver,
}

impl DzahuiWindowBuilder {
    /// Creates default instance.
    fn new<F>(location: F) -> Self
    where
        F: AsRef<str>,
    {
        Self {
            geometry_vertex_shader: None,
            geometry_fragment_shader: None,
            character_set: None,
            integration_iteration: None,
            text_vertex_shader: None,
            text_fragment_shader: None,
            opengl_version: Some((3, 3)),
            vertex_selector: None,
            camera: Camera::builder(),
            mesh: Mesh::builder(location),
            mesh_dimension: MeshDimension::Two,
            height: Some(600),
            width: Some(800),
            solver: Solver::None,
        }
    }
    /// Changes geometry shader.
    pub fn with_geometry_shader<A, B>(self, vertex_shader: A, fragment_shader: B) -> Self
    where
        A: AsRef<str>,
        B: AsRef<str>,
    {
        Self {
            geometry_vertex_shader: Some(vertex_shader.as_ref().to_string()),
            geometry_fragment_shader: Some(fragment_shader.as_ref().to_string()),
            ..self
        }
    }
    /// Changes text shader.
    pub fn with_text_shader<A, B>(self, vertex_shader: A, fragment_shader: B) -> Self
    where
        A: AsRef<str>,
        B: AsRef<str>,
    {
        Self {
            text_vertex_shader: Some(vertex_shader.as_ref().to_string()),
            text_fragment_shader: Some(fragment_shader.as_ref().to_string()),
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
    pub fn with_opengl_version(self, opengl_version: (u8, u8)) -> Self {
        Self {
            opengl_version: Some(opengl_version),
            ..self
        }
    }
    /// Changes angle to determine selected vertex.
    pub fn with_vertex_angle(self, angle: f32) -> Self {
        Self {
            vertex_selector: Some(angle),
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
    /// Changes camera movement arround object being targeted
    pub fn with_sensitivity(self, sensitivity: f32) -> Self {
        Self {
            camera: self.camera.with_sensitivity(sensitivity),
            ..self
        }
    }
    /// Changes mesh dimension to 3D (originally in 2D)
    pub fn with_mesh_in_3d(self) -> Self {
        Self {
            mesh_dimension: MeshDimension::Three,
            ..self
        }
    }
    /// Changes mesh dimension to 1D (originally in 2D)
    pub fn with_mesh_in_1d(self) -> Self {
        Self {
            mesh_dimension: MeshDimension::One,
            ..self
        }
    }
    /// Makes diffusion solver simulation
    pub fn solve_1d_diffussion(self, boundary_conditions: [f64; 2], mu: f64, b: f64) -> Self {
        Self {
            solver: Solver::DiffussionSolverTimeIndependent(boundary_conditions, mu, b),
            mesh_dimension: MeshDimension::One,
            ..self
        }
    }
    // Makes time-dependant diffusion solver simulation
    pub fn solve_1d_time_dependant_diffussion(self, boundary_conditions: [f64; 2], initial_conditions: Vec<f64>, mu: f64, b: f64) -> Self {
        Self {
            solver: Solver::DiffussionSolverTimeDependent(boundary_conditions, initial_conditions, mu, b),
            mesh_dimension: MeshDimension::One,
            ..self
        }
    }
    /// Sets integration iteration
    pub fn with_integration_iteration(self, integration_iteration: usize) -> Self {
        Self {
            integration_iteration: Some(integration_iteration),
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
        let window_builder = WindowBuilder::new()
            .with_title("Dzahui")
            .with_inner_size(PhysicalSize {
                height: self.height.unwrap(),
                width: self.width.unwrap(),
            })
            .with_resizable(true);

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
        let context = unsafe { context.make_current().unwrap() };

        // Loading OpenGL functions. Only done once
        gl::load_with(&|s: &str| context.get_proc_address(s));
        // GL Viewport
        unsafe {
            gl::Viewport(
                0,
                0,
                self.width.unwrap() as i32,
                self.height.unwrap() as i32,
            );
            gl::Enable(gl::DEPTH_TEST);
        }

        // Use text_shaders chosen
        let vertex_shader: String = if let Some(vertex_shader) = self.text_vertex_shader {
            vertex_shader
        } else {
            "./assets/text_vertex_shader.vs".to_string()
        };

        let fragment_shader: String = if let Some(fragment_shader) = self.text_fragment_shader {
            fragment_shader
        } else {
            "./assets/text_fragment_shader.fs".to_string()
        };

        let text_shader = Shader::new(vertex_shader, fragment_shader).unwrap();

        // Use geometry_shaders chosen
        let vertex_shader: String = if let Some(vertex_shader) = self.geometry_vertex_shader {
            vertex_shader
        } else {
            "./assets/geometry_vertex_shader.vs".to_string()
        };

        let fragment_shader: String = if let Some(fragment_shader) = self.geometry_fragment_shader {
            fragment_shader
        } else {
            "./assets/geometry_fragment_shader.fs".to_string()
        };

        let geometry_shader = Shader::new(vertex_shader, fragment_shader).unwrap();

        // Creating mesh based on initial provided file.
        let mesh = match self.mesh_dimension {
            MeshDimension::One => self.mesh.build_mesh_1d(),
            MeshDimension::Two => self.mesh.build_mesh_2d(),
            MeshDimension::Three => self.mesh.build_mesh_3d(),
        }
        .unwrap();

        // Camera created with selected configuration via shortcut functions.
        let camera = self.camera.build(
            mesh.max_length as f32,
            self.height.unwrap(),
            self.width.unwrap(),
        );

        // Vertex selector (cone)
        let angle = if let Some(angle) = self.vertex_selector {
            angle
        } else {
            3.0
        };
        let vertex_selector = Cone::new(
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
            angle,
        );

        // set integration precision
        let integration_iteration = if let Some(integration_iteration) = self.integration_iteration {
            integration_iteration
        } else {
            150
        };

        // Default character set
        let character_set_file = if let Some(set_file) = self.character_set {
            set_file
        } else {
            "assets/dzahui-font_2.fnt".to_string()
        };
        let character_set = CharacterSet::new(&character_set_file);

        // Start clock for delta time
        let timer = Instant::now();

        DzahuiWindow {
            context,
            timer,
            geometry_shader,
            text_shader,
            vertex_selector,
            character_set,
            integration_iteration,
            mesh,
            camera,
            width: self.width.unwrap(),
            height: self.height.unwrap(),
            event_loop: Some(event_loop),
            mouse_coordinates: Point2::new(0.0, 0.0),
            solver: self.solver,
        }
    }
}

impl DzahuiWindow {
    /// Creates a new default builder.
    pub fn builder<F>(location: F) -> DzahuiWindowBuilder
    where
        F: AsRef<str>,
    {
        DzahuiWindowBuilder::new(location)
    }

    /// To restart timer of window in case is needed.
    pub fn restart_timer(&mut self) {
        self.timer = Instant::now();
    }

    /// Callback to change mouse coordinates.
    pub fn update_mouse_coordinates(&mut self, x: f32, y: f32) {
        self.mouse_coordinates.x = x;
        self.mouse_coordinates.y = y;
    }

    /// Callback that changes wether camera can be edited by user input or not.
    fn activate_view_change(&mut self, state: ElementState) {
        match state {
            ElementState::Pressed => self.camera.active_view_change = true,
            ElementState::Released => self.camera.active_view_change = false,
        }
    }

    /// Callback to obtain vertex intersection with click produced cone.
    fn get_selected_vertex(&mut self) {
        self.vertex_selector.change_from_mouse_position(
            &self.mouse_coordinates,
            &self.camera.projection_matrix,
            self.width,
            self.height,
        );
        let sel_vec = self
            .vertex_selector
            .obtain_nearest_intersection(&self.mesh.vertices, &self.camera.view_matrix);
        println!("{:?}", sel_vec);
    }

    /// Callback to change camera view matrix based on user motion.
    fn change_camera_view(&mut self, x: f32, y: f32) {
        let x_offset = x * self.camera.camera_sensitivity;
        let y_offset = y * self.camera.camera_sensitivity;
        self.camera.theta -= y_offset;
        self.camera.phi -= x_offset;

        // Do not allow 0 (or 180) degree angle (coincides with y-axis).
        if self.camera.theta < 1.0 {
            self.camera.theta = 1.0;
        } else if self.camera.theta > 179.0 {
            self.camera.theta = 179.0;
        }

        // update position
        self.camera.camera_position = Point3::new(
            self.camera.theta.to_radians().sin() * self.camera.phi.to_radians().sin(),
            self.camera.theta.to_radians().cos(),
            self.camera.theta.to_radians().sin() * self.camera.phi.to_radians().cos(),
        ) * self.camera.radius;

        // generate new matrix
        self.camera.modify_view_matrix();
    }

    /// Callback to resize window and change dimensions.
    fn resize_window(&mut self, new_size: PhysicalSize<u32>) {
        self.context.resize(new_size);
        self.height = new_size.height;
        self.width = new_size.width;
    }

    /// # General Information
    ///
    /// Run window with a mesh and an event loop. Consumes every object.
    ///
    /// # Parameters
    ///
    /// * `self` - A window instance.
    /// * `mesh` - A file to draw a mesh from.
    ///
    pub fn run(mut self) {
        self.restart_timer();
        let mut counter = 0;
        let mut fps = 0;
        let mut prev_time = 0;
        let mut current_time = 0;

        // Obtaining Event Loop is necessary since `event_loop.run()` consumes it alongside window if let inside struct instance.
        let event_loop = Option::take(&mut self.event_loop).unwrap();

        // Generating differential equation solver.
        let mut solver: Box<dyn DiffEquationSolver> = match self.solver {
            Solver::DiffussionSolverTimeIndependent(boundary_conditions, mu, b) => {
                Box::new(DiffussionSolverTimeIndependent::new(
                    boundary_conditions,
                    self.mesh.filter_for_solving_1d().to_vec(),
                    mu,
                    b,
                ))
            }

            Solver::DiffussionSolverTimeDependent(boundary_conditions, ref initial_conditions, mu, b) => {
                Box::new(DiffussionSolverTimeDependent::new(
                    boundary_conditions,
                    initial_conditions.clone(),
                    self.mesh.filter_for_solving_1d().to_vec(),
                    mu,
                    b,
                ).unwrap())
            }

            _ => {panic!()}
        };

        // Send mesh info: mesh structure and vertices to create body on each one.
        self.mesh.setup().unwrap();
        self.mesh.send_to_gpu().unwrap();

        // Setup character set info.
        // Maybe need to change shader (but I think shaders and binders are independent, so leave it like this for now).
        self.character_set.setup().unwrap();
        self.character_set.setup_texture().unwrap();
        self.character_set.send_to_gpu();

        // Use geometry shader.
        self.geometry_shader.use_shader();
        // translation for mesh to always be near (0,0).
        self.geometry_shader
            .set_mat4("model", self.mesh.get_model_matrix());
        self.geometry_shader
            .set_mat4("view", &self.camera.view_matrix);
        self.geometry_shader
            .set_mat4("projection", &self.camera.projection_matrix);

        // Use text shader to assign matrices.
        self.text_shader.use_shader();

        let model_mat =
            CharacterSet::matrix_for_screen(0.0, 0.0, &self.camera, self.height, self.width);

        self.text_shader.set_mat4("model", &model_mat);
        self.text_shader
            .set_mat4("projection", &self.camera.projection_matrix);
        self.text_shader.set_mat4("view", &Matrix4::identity());

        event_loop.run(move |event, _, control_flow| {
            // Temporal FPS counter
            current_time = self.timer.elapsed().as_millis();
            if current_time - prev_time >= 1000 {
                prev_time = current_time;
                fps = counter;
                counter = 0;
            }

            match event {
                Event::LoopDestroyed => (), // subscribing to events occurs here

                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(physical_size) => self.resize_window(physical_size),

                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                    WindowEvent::CursorMoved {
                        device_id: _,
                        position,
                        ..
                    } => self.update_mouse_coordinates(position.x as f32, position.y as f32),

                    WindowEvent::KeyboardInput { input, .. } => match input.scancode {
                        1 => *control_flow = ControlFlow::Exit,
                        _ => (),
                    },

                    _ => (),
                },

                Event::DeviceEvent {
                    device_id: _,
                    event,
                } => match event {
                    DeviceEvent::Button { button, state } => match button {
                        2 => self.activate_view_change(state),
                        1 => {
                            if let ElementState::Pressed = state {
                                self.get_selected_vertex();
                            }
                        }
                        _ => {}
                    },

                    DeviceEvent::MouseMotion { delta: (x, y) } => {
                        match self.camera.active_view_change {
                            true => {
                                self.change_camera_view(x as f32, y as f32);
                            }
                            false => (),
                        }
                    }

                    _ => {}
                },

                Event::RedrawRequested(_) => (),

                _ => (),
            }

            // Render
            unsafe {
                // Update to some color
                // Clear Screen
                gl::ClearColor(0.33, 0.33, 0.33, 0.8);
                gl::Clear(gl::COLOR_BUFFER_BIT);
                gl::Clear(gl::DEPTH_BUFFER_BIT);
            }

            let solution = solver.solve(self.integration_iteration, 0.01).unwrap();
                
            println!("{:?}", solution);
            // updating colors. Only one time per vertex should it be updated (that is, every 6 steps).
            self.mesh.update_gradient_1d(solution.iter().map(|x| x.abs()).collect());
                

            // Text shader to draw text
            self.text_shader.use_shader();

            self.character_set.bind_all().unwrap();
            self.character_set.draw_text(format!(
                "x: {}, y: {}, FPS: {}",
                self.mouse_coordinates.x, self.mouse_coordinates.y, fps
            ));
            self.character_set.unbind_texture().unwrap();

            // Geometry shader to draw mesh
            self.geometry_shader.use_shader();
            self.geometry_shader
                .set_mat4("view", &self.camera.view_matrix);

            self.mesh.bind_vao().unwrap();
            self.mesh.draw(&self).unwrap();
            // Need to change old and new buffer to redraw
            self.context.swap_buffers().unwrap();
            counter += 1;
        })
    }
}
