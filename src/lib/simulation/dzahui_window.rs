// Internal dependencies
use crate::{mesh::{mesh_builder::{MeshBuilder, MeshDimension}, Mesh},
    solvers::{Solver, DiffussionSolverTimeDependent, DiffussionSolverTimeIndependent,
        solver_trait::DiffEquationSolver, DiffussionParamsTimeDependent, DiffussionParamsTimeIndependent, NoSolver, StaticPressureSolver, StokesParams1D
    }, Error, writer::{self, Writer}, logger
};
use super::{shader::Shader, drawable::{text::CharacterSet, binder::{Bindable, Drawable}}, camera::{cone::Cone, Camera, CameraBuilder}};


// External dependencies
use glutin::{
    dpi::PhysicalSize,
    event::{DeviceEvent, ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
    Api, ContextBuilder, ContextWrapper, GlProfile, GlRequest, PossiblyCurrent,
};
use cgmath::{Matrix4, Point2, Point3, SquareMatrix, Vector3};
use std::{time::Instant, sync::mpsc::{self, SyncSender}};
use gl;


/// # General Information
///
/// DzahuiWindow holds every important component to create an instancec of a simulator. Only one instance should be active at once.
///
/// # Fields
///
/// * `context` - Holds an *instance* of OpenGL. This normally means that all configuration associated with rendering is stored here. Only one context is allowed
/// * `geometry_shader` - Geometry_shaders to compile and use. Responsible for mesh drawing
/// * `event_loop` - To obtain user input in window and refresh window
/// * `mouse_coordinates` - Current coordinates of mouse
/// * `initial_time_step` - When solving a time-dependent problem and not specifiying a time, an initial time should be given while enough information is collected
/// to use framerate
/// * `character_set` - Set of characters to draw on screen
/// * `integration_iteration` - Amount of terms to approximate integral
/// * `height` - Height of window created
/// * `width` - Width of window created
/// * `vertex_selector` - A cone to interact with the screen using the mouse
/// * `text_shader` - Text shaders to compile and use. Responsible for text rendering
/// * `window_text_scale` - Scale of text in front of window. This text does not change with camera view
/// * `timer` - Gives current time since creation of window. Call with `timer.elapsed()`
/// * `camera` - Camera configuration creates view and projetion matrices, which directly tells OpenGL what to and not to render
/// * `solver` - Solver enum representing the kind of equation to simmulate
/// * `time_step` - How much to forward a time-dependent solution 
/// * `mesh` - A mesh to draw to screen. Represents an object tessellated into triangles/traingular prisms
/// * `write_location` - Where to write values from solved equation of needed
/// * `file_prefix`- If writing files require a prefix to identify them
///
pub struct DzahuiWindow {
    context: ContextWrapper<PossiblyCurrent, Window>,
    pub(crate) geometry_shader: Shader,
    event_loop: Option<EventLoop<()>>,
    mouse_coordinates: Point2<f32>,
    initial_time_step: Option<f64>,
    character_set: CharacterSet,
    integration_iteration: usize,
    pub(crate) height: u32,
    pub(crate) width: u32,
    vertex_selector: Cone,
    text_shader: Shader,
    window_text_scale: f32,
    pub timer: Instant,
    camera: Camera,
    solver: Solver,
    time_step: f64,
    mesh: Mesh,
    write_location: String,
    file_prefix: String,
}

/// # General Information
///
/// The window builder. When using function `builder` in **DzahuiWindow** without parameters a sensible default is obtained.
///
/// # Fields
///
/// * `geometry_fragment_shader` - Shader used to render triangulated 3D and 2D meshes. Defaults to assets/geometry_fragment_shader.fs
/// * `geometry_vertex_shader` - Shader used to render text. Defaults to assets/geometry_vertex_shader.vs
/// * `text_fragment_shader` - Shader used to render triangulated 3D and 2D meshes. Defaults to assets/text_fragment_shader.fs
/// * `text_vertex_shader` - Shader used to render text. Defaults to assets/text_vertex_shader.vs
/// * `height_multiplier` - Makes height of mesh bigger. Useful for 1D mesh.
/// * `integration_iteration` - Amount of elements to sum to approximate integral
/// * `opengl_version` - opengl version to use. Tested with 3.3, latter versions should work too
/// * `initial_time_step` - When solving a time-dependent problem and not specifiying a time, an initial time should be given while enough information is collected
/// to use framerate
/// * `window_text_scale` - Scale of text in front of window. This text does not change with camera view
/// * `mesh_dimension` - Dimension of mesh to build. Used to process certain elements of solution
/// * `character_set` - Set of characters to draw on screen
/// * `vertex_selector` - Angle for the cone that casts mouse coordinates to 3d world and selects vertices
/// * `time_step` - How much to advance a time-dependent solution 
/// * `camera` - A CameraBuilder. Certain properties can be changend via this structure's methods
/// * `height` - Height of window. Defaults to 600 px.
/// * `width` - Width of window. Defaults to 800 px.
/// * `mesh` - A MeshBuilder. Certain properties can be changed via this structre's methods
/// * `solver` - An enum representing the equation to be solved
/// * `write_location` - Where to write values from solved equation of needed. Will be chosen automatically if None
/// * `file_prefix`- If writing files require a prefix to identify them. Will be chosen automatically if None
///
#[derive(Debug)]
pub struct DzahuiWindowBuilder {
    geometry_fragment_shader: Option<String>,
    geometry_vertex_shader: Option<String>,
    text_fragment_shader: Option<String>,
    text_vertex_shader: Option<String>,
    height_multiplier: Option<f64>,
    integration_iteration: Option<usize>,
    opengl_version: Option<(u8, u8)>,
    initial_time_step: Option<f64>,
    window_text_scale: Option<f32>,
    mesh_dimension: MeshDimension,
    character_set: Option<String>,
    vertex_selector: Option<f32>,
    time_step: Option<f64>,
    camera: CameraBuilder,
    height: Option<u32>,
    width: Option<u32>,
    mesh: MeshBuilder,
    solver: Solver,
    write_location: Option<String>,
    file_prefix: Option<String>
}

impl DzahuiWindowBuilder {
    /// Creates default instance.
    fn new<F>(location: F) -> Self
    where
        F: AsRef<str>,
    {
        // Spawning logger
        logger::spawn(log::LevelFilter::Info, "dzahui").unwrap();
        
        Self {
            mesh_dimension: MeshDimension::Two,
            geometry_fragment_shader: None,
            mesh: Mesh::builder(location),
            geometry_vertex_shader: None,
            integration_iteration: None,
            opengl_version: Some((3, 3)),
            text_fragment_shader: None,
            camera: Camera::builder(),
            text_vertex_shader: None,
            height_multiplier: None,
            window_text_scale: None,
            initial_time_step: None,
            vertex_selector: None,
            solver: Solver::None,
            character_set: None,
            height: Some(600),
            width: Some(800),
            time_step: None,
            write_location: None,
            file_prefix: None
        }
    }
    /// Changes geometry shader.
    pub fn with_geometry_shader<A, B>(self, vertex_shader: A, fragment_shader: B) -> Self
    where
        A: AsRef<str>,
        B: AsRef<str>,
    {
        log::warn!("Changing shaders for geometry can have undesired results. Proceed with caution");
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
        log::warn!("Changing shaders for text can have undesired results. Proceed with caution");
        Self {
            text_vertex_shader: Some(vertex_shader.as_ref().to_string()),
            text_fragment_shader: Some(fragment_shader.as_ref().to_string()),
            ..self
        }
    }
    /// Makes height larger for 1D mesh
    pub fn enable_height_multiplier(self, height_multiplier: f64) -> Self {
        Self {
            height_multiplier: Some(height_multiplier),
            ..self
        }
    }
    /// Changes height and width.
    pub fn with_height_and_width(self, height: u32, width: u32) -> Self {

        if height == 0 || width == 0 {
            panic!("Dimensions cannot be zero!")
        }

        Self {
            height: Some(height),
            width: Some(width),
            ..self
        }
    }
    /// Changes opengl version.
    pub fn with_opengl_version(self, opengl_version: (u8, u8)) -> Self {
        if opengl_version.0 !=3 && opengl_version.1 != 3 {
            log::warn!("Not using OpenGL version 3.3 can have undesired consequences")
        }
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
        log::warn!("Changing distance to object could block simulation view");
        Self {
            camera: self.camera.change_distance_to_object(radius),
            ..self
        }
    }
    /// Changes object being targeted
    pub fn camera_with_target(self, x: f32, y: f32, z: f32) -> Self {
        log::warn!("Changing target could block simulation view");
        Self {
            camera: self.camera.with_target(x, y, z),
            ..self
        }
    }
    /// Changes camera position in a sphere with center `camera_target`
    pub fn with_camera_position(self, theta: f32, phi: f32) -> Self {
        log::warn!("Changing camera position could block simulation view");
        Self {
            camera: self.camera.with_camera_position(theta, phi),
            ..self
        }
    }
    /// Changes fov when using projection matrix
    pub fn with_fov(self, fov: f32) -> Self {
        log::info!("Changing fov could give you a doom-like experience");
        Self {
            camera: self.camera.with_fov(fov),
            ..self
        }
    }
    /// Changes camera movement arround object being targeted
    pub fn with_sensitivity(self, sensitivity: f32) -> Self {
        log::warn!("Changing camera sensitivity can make harder to control simulation perspective");
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
    pub fn solve_1d_diffussion(self, params: DiffussionParamsTimeIndependent) -> Self {
        Self {
            solver: Solver::DiffussionSolverTimeIndependent(params),
            mesh_dimension: MeshDimension::One,
            ..self
        }
    }
    /// Makes time-dependant diffusion solver simulation
    pub fn solve_1d_time_dependent_diffussion(self, params: DiffussionParamsTimeDependent) -> Self {
        Self {
            solver: Solver::DiffussionSolverTimeDependent(params),
            mesh_dimension: MeshDimension::One,
            ..self
        }
    }
    /// Makes Stokes time-independent solver simulation
    pub fn solve_1d_stokes(self, params: StokesParams1D) -> Self {
        Self {
            solver: Solver::Stokes1DSolver(params),
            mesh_dimension: MeshDimension::One,
            ..self
        }
    }
    /// Makes Stokes time-independent solver simulation with alias StaticPressureSolver
    pub fn solve_static_pressure(self, params: StokesParams1D) -> Self {
        Self {
            solver: Solver::Stokes1DSolver(params),
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
    /// Sets time step for time-dependant solutions
    pub fn with_time_step(self, time_step: f64) -> Self {
        Self {
            time_step: Some(time_step),
            ..self
        }
    }
    /// Size of text present on screen. Default is chosen otherwise, which may not be good for every scenario
    pub fn with_window_text_scale(self, window_text_scale: f32) -> Self {
        log::warn!("Text rendering can block simulation it it's too big or can dissapear if it's too small");
        Self {
            window_text_scale: Some(window_text_scale),
            ..self
        }
    }
    /// Initial time step when simulation on real time
    pub fn with_initial_time_step(self, initial_time_step: f64) -> Self {
        if let Some(_) = self.time_step {
            log::warn!("time_step is set, therefore initial_time_step should not be set since simulation will not occur in real-time");
        }
        log::warn!("This could result in a non-convergent solution");
        Self {
            initial_time_step: Some(initial_time_step),
            ..self
        }
    }
    /// Set file location. If let None, a predetermined will be chosen later
    pub fn set_file_location<A: AsRef<str>>(self, write_location: A) -> Self {
        Self {
            write_location: Some(write_location.as_ref().to_string()),
            ..self
        }
    }
    /// Set file prefix. If let None, a predetermined will be chosen later
    pub fn set_file_prefix<A: AsRef<str>>(self, file_prefix: A) -> Self {
        Self {
            file_prefix: Some(file_prefix.as_ref().to_string()),
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
    /// Then an OpenGL version is assigned based on builder.
    /// Event loop is generated and, alongside window, made current context.
    /// OpenGL functions are made available and viewport for OpenGL is set.
    /// Geometry and Text shaders are created.
    /// A new instance of Mesh is created for later use.
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
    
        // Will never be None
        let height = self.height.unwrap();
        let width = self.width.unwrap();

        let window_builder = WindowBuilder::new()
            .with_title("Dzahui")
            .with_inner_size(PhysicalSize {
                height,
                width,
            })
            .with_resizable(true);

        let opengl_version = GlRequest::Specific(Api::OpenGl, self.opengl_version.unwrap());

        // Generating event_loop to be used
        let event_loop = EventLoop::new();

        // Creating context to use in application
        let context = match ContextBuilder::new().
        with_gl(opengl_version).
        // core GL profile
        // Future compatible functions. Not backwards compatible (no previous versions of openGL).
        with_gl_profile(GlProfile::Core).
        with_vsync(true).
        build_windowed(window_builder, &event_loop) {
            Ok(w) => w,
            Err(e) => panic!("Error on window creation: {}",e)
        };

        // The latest instance becomes the current context always
        let context = unsafe { 
            match context.make_current() {
                Ok(context) => context,
                Err(e) => panic!("Initializing context for window failed!: {}",e.1)
            }
        };
        log::info!("Window context created");

        // Loading OpenGL functions. Only done once
        gl::load_with(&|s: &str| context.get_proc_address(s));
        // GL Viewport
        unsafe {
            gl::Viewport(
                0,
                0,
                // Cannot fail
                width as i32,
                height as i32,
            );
            gl::Enable(gl::DEPTH_TEST);
        }
        log::info!("OpenGL functions loaded");

        // Use text_shaders chosen
        let vertex_shader: String = if let Some(vertex_shader) = self.text_vertex_shader {
            vertex_shader
        } else {
            log::info!("Using default text shaders");
            "./assets/text_vertex_shader.vs".to_string()
        };

        let fragment_shader: String = if let Some(fragment_shader) = self.text_fragment_shader {
            fragment_shader
        } else {
            "./assets/text_fragment_shader.fs".to_string()
        };

        let text_shader = match Shader::new(vertex_shader, fragment_shader) {
            Ok(shader) => shader,
            Err(e) => panic!("Error on text shader creation!: {}", e)
        };

        // Use geometry_shaders chosen
        let vertex_shader: String = if let Some(vertex_shader) = self.geometry_vertex_shader {
            vertex_shader
        } else {
            log::info!("Using default geometry shaders");
            "./assets/geometry_vertex_shader.vs".to_string()
        };

        let fragment_shader: String = if let Some(fragment_shader) = self.geometry_fragment_shader {
            fragment_shader
        } else {
            "./assets/geometry_fragment_shader.fs".to_string()
        };

        let geometry_shader = match Shader::new(vertex_shader, fragment_shader) {
            Ok(shader) => shader,
            Err(e) => panic!("Error on geometry shader creation!: {}",e)
        };

        // Creating mesh based on initial provided file.
        let mesh = match match self.mesh_dimension {
            MeshDimension::One => {
                log::info!("Creating a 1D Mesh");
                self.mesh.build_mesh_1d(self.height_multiplier)
            },
            MeshDimension::Two => {
                log::info!("Creating a 2D Mesh");
                self.mesh.build_mesh_2d()
            },
            MeshDimension::Three => {
                log::info!("Creating a 3D Mesh");
                self.mesh.build_mesh_3d()
            },
        } {
            Ok(mesh) => mesh,
            Err(e) => panic!("Error while creating mesh!: {}", e)
        };

        let window_text_scale = if let Some(sc) = self.window_text_scale {
            log::info!("Text scale is: {}",sc);
            sc
        } else {
            log::info!("Text scale is: 0.0001");
            0.0001
        };

        // Camera created with selected configuration via shortcut functions.
        let camera = self.camera.build(
            mesh.max_length as f32,
            height,
            width
        );
        log::info!("Camera created");

        // Vertex selector (cone)
        let angle = if let Some(angle) = self.vertex_selector {
            log::info!("Angle for vertex selector is: {}",angle);
            angle
        } else {
            log::info!("Angle for vertex selector is: 3.0");
            3.0
        };
        let vertex_selector = Cone::new(
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
            angle,
        );
        log::info!("Vertex selector created");

        // set integration precision
        let integration_iteration = if let Some(integration_iteration) = self.integration_iteration {
            log::info!("Integration iteration is {}",integration_iteration);
            integration_iteration
        } else {
            log::info!("Integration iteration is 150");
            150
        };

        // Set initial value for time step.
        // When time step is provided, it's used. When time step is not provided but initial time step is (mainly because of real-time simulation purposes),
        // It is used as initial value. When none are provided, 0.000001 is used by default
        let time_step = if let Some(time_step) = self.time_step {
            time_step
        } else {
            if let Some(initial_time_step) = self.initial_time_step {
                initial_time_step
            } else {
                0.000001
            }
        };

        // Default character set
        let character_set_file = if let Some(set_file) = self.character_set {
            set_file
        } else {
            "assets/dzahui-font_2.fnt".to_string()
        };
        let character_set = match CharacterSet::new(&character_set_file) {
            Ok(chs) => chs,
            Err(e) => panic!("Error while creating character set!: {}",e)
        };
        log::info!("Character set loaded");

        // Writing location setting
        let write_location = if let Some(s) = self.write_location {
            s
        } else {
            "./saved".to_string()
        };

        // File prefix setting
        let file_prefix = if let Some(s) = self.file_prefix {
            s
        } else {
            "result".to_string()
        };

        // Start clock for delta time
        let timer = Instant::now();

        DzahuiWindow {
            context,
            timer,
            geometry_shader,
            window_text_scale,
            text_shader,
            vertex_selector,
            character_set,
            integration_iteration,
            mesh,
            time_step,
            camera,
            width,
            height,
            write_location,
            file_prefix,
            event_loop: Some(event_loop),
            mouse_coordinates: Point2::new(0.0, 0.0),
            solver: self.solver,
            initial_time_step: self.initial_time_step,

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
    fn get_selected_vertex(&mut self) -> Result<(),Error> {
        
        self.vertex_selector.change_from_mouse_position(
            &self.mouse_coordinates,
            &self.camera.projection_matrix,
            self.width,
            self.height,
        )?;
        
        let sel_vec = self
            .vertex_selector
            .obtain_nearest_intersection(&self.mesh.vertices, &self.camera.view_matrix);
        println!("{:?}", sel_vec);
        Ok(())
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

    /// Callback to resize window.
    fn resize_window(&mut self, new_size: PhysicalSize<u32>) {
        self.context.resize(new_size);
        self.height = new_size.height;
        self.width = new_size.width;
    }

    /// Send information of vertices to be written
    fn send_vertex_info(&self, info: Vec<f64>, sender: &SyncSender<Vec<f64>>) {
        match sender.send(info) {
            Err(e) => panic!("Error while communicating between threads. Report this to the deveoper!: {}",e),
            _ => {}
        }
    }

    /// # General Information
    ///
    /// Run window with a mesh and an event loop. Consumes every object.
    /// Generates actual solver based on solver enum.
    /// Sets mesh and text.
    /// 
    ///
    /// # Parameters
    ///
    /// * `mut self` - A window instance.
    ///
    pub fn run(mut self) {

        self.restart_timer();
        let mut counter = 0;
        let mut fps = 0;
        let mut prev_time = 0;
        // To know wether writer can be called again or not
        let mut writer_sleep = 0;

        //set up objects for thread writer
        let (tx, rx) = mpsc::sync_channel(3);
        
        // set writer
        let writer = match self.solver {
            Solver::DiffussionSolverTimeDependent(_) => {
                Writer::new(rx, &self.write_location, &self.file_prefix, ["v_x"], true)
            },
            Solver::DiffussionSolverTimeIndependent(_) => {
                Writer::new(rx, &self.write_location, &self.file_prefix, ["v_x"], true)
            },
            Solver::Stokes1DSolver(_) => {
                Writer::new(rx,&self.write_location, &self.file_prefix, ["p"],true)
            },
            Solver::Stokes2DSolver(_) => {
                Writer::new(rx,&self.write_location, &self.file_prefix,["v_x","v_y","p"],true)
            }
            Solver::None => {
                Writer::new(rx, &self.write_location, &self.file_prefix, [""], false)
            }
        };

        let writer = match writer {
            Ok(w) => w,
            Err(e) => panic!("Unable to create writer to record values to files!: {}",e)
        };
        // copy of timer for new thread
        let timer_copy = self.timer.clone();

        // sending writer to thread and start execution
        writer::spawn(writer, timer_copy);
        log::info!("Writer has been set in: {}",self.write_location);
        log::info!("Files will have prefix: {}",self.file_prefix);
        

        // Obtaining Event Loop is necessary since `event_loop.run()` consumes it alongside window if let inside struct instance.
        let event_loop = Option::take(&mut self.event_loop).unwrap();

        // Generating differential equation solver.
        let mut solver: Box<dyn DiffEquationSolver> = match self.solver {

            Solver::DiffussionSolverTimeIndependent(ref params) => {
                
                let diffussion_solver = DiffussionSolverTimeIndependent::new(
                    &params,
                    self.mesh.filter_for_solving_1d().to_vec(),
                    self.integration_iteration);

                    
                match diffussion_solver {
                    Ok(d) => {
                        log::info!("Diffussion solver with time independence created");
                        Box::new(d)
                    },
                    Err(error) => panic!("Error creating instance of DiffussionSolverTimeIndependent!: {}",error)
                }

            },

            Solver::DiffussionSolverTimeDependent(ref params) => {
                
                let diffussion_solver = DiffussionSolverTimeDependent::new(
                    &params,
                    self.mesh.filter_for_solving_1d().to_vec(),
                    self.integration_iteration,
                );

                match diffussion_solver {
                    Ok(d) => {
                        log::info!("Diffussion solver with time dependence created");
                        Box::new(d)
                    },
                    Err(error) => panic!("Error creating instance of DiffussionSolverTimeDependent!: {}",error)
                }
            },

            Solver::Stokes1DSolver(ref params) => {
                
                let stokes_1d_solver = StaticPressureSolver::new(
                    &params,
                    self.mesh.filter_for_solving_1d().to_vec(),
                    self.integration_iteration
                );

                match stokes_1d_solver {
                    Ok(n) => {
                        log::info!("Stokes solver in 1D with no time dependency created");
                        Box::new(n)
                    },
                    Err(error) => panic!("Error creating instance of StokesSolver1D!: {}",error)
                }
            },

            Solver::Stokes2DSolver(ref _params) => {
                panic!("Not implemented yet!")
            }

            Solver::None => {
                log::info!("No solver selected. Program will display Mesh");
                Box::new(NoSolver())
            }
        };

        // Send mesh info: mesh structure and vertices to create body on each one.
        if let Err(e) = self.mesh.setup() {
            panic!("Error while setting up mesh on GPU!: {}",e)
        };
        if let Err(e) = self.mesh.send_to_gpu() {
            panic!("Error while sending mesh to GPU!: {}",e)
        }
        log::info!("Mesh info has been set up");

        // Setup character set info.
        if let Err(e) = self.character_set.setup() {
            panic!("Error while setting up character set to write on screen!: {}",e)
        }
        if let Err(e) = self.character_set.setup_texture() {
            panic!("Error while setting up texture for character set!: {}",e)
        }
        self.character_set.send_to_gpu();
        log::info!("Characters for writing have been set up");

        // Use geometry shader.
        self.geometry_shader.use_shader();
        // translation for mesh to always be near (0,0).
        if let Err(e) = self.geometry_shader
            .set_mat4("model", self.mesh.get_model_matrix()) {
                panic!("Unable to set model matrix for geometry!: {}",e)
            }
        if let Err(e) = self.geometry_shader
            .set_mat4("view", &self.camera.view_matrix) {
                panic!("Unable to set view matrix for geometry!: {}",e)
            }
        if let Err(e) = self.geometry_shader
            .set_mat4("projection", &self.camera.projection_matrix) {
                panic!("Unable to set projection matrix for geometry!: {}",e)
            }
        log::info!("Matrices for Mesh visualization set up");

        // Use text shader to assign matrices.
        self.text_shader.use_shader();

        let model_mat =
            match CharacterSet::matrix_for_screen(0.0, 0.0,
                &self.camera.projection_matrix, self.height, self.width, self.window_text_scale) {
                
                Ok(mat) => mat,
                Err(e) => panic!("Matrix for character set not created properly!: {}",e)
            
            };

        if let Err(e) = self.text_shader.set_mat4("model", &model_mat) {
            panic!("Unable to set model matrix for text!: {}",e)
        }
        if let Err(e) = self.text_shader
            .set_mat4("projection", &self.camera.projection_matrix) {
                panic!("Unable to set projection  matrix for text!: {}",e)
            }
        if let Err(e) = self.text_shader.set_mat4("view", &Matrix4::identity()) {
            panic!("Unable to set view matrix for text shader!: {}",e)
        }
        log::info!("Matrices for Character visualization set up");

        // Keep last result
        let mut solution: Vec<f64> = vec![];
        // to fill or not mesh
        let mut fill = true;

        event_loop.run(move |event, _, control_flow| {

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
                        53 => *control_flow = ControlFlow::Exit,
                        1 => {
                            match input.state {
                                
                                ElementState::Pressed => {
                                    let current_time = self.timer.elapsed().as_millis();
                                    // Block many succesive calls to savde data (can do 5 per second)
                                    if current_time - writer_sleep > 200 {
                                        writer_sleep = current_time; 
                                        self.send_vertex_info(solution.clone(), &tx)
                                    }
                                },
                                
                                _ => {}
                            }

                        },
                        17 => {
                            match input.state {

                                ElementState::Pressed => {
                                    fill = false;
                                },
                                
                                _ => fill = true

                            }
                        }
                        _ => {},
                    },

                    _ => (),
                },

                Event::DeviceEvent {
                    device_id: _,
                    event,
                } => match event {
                    DeviceEvent::Button { button, state } =>
                        match button {
                        2 => self.activate_view_change(state),
                        0 => self.activate_view_change(state),
                        1 => {
                            if let ElementState::Pressed = state {
                                if let Err(e) = self.get_selected_vertex() {
                                    panic!("Error while using cone vertex selector!: {}",e)
                                }
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

                Event::MainEventsCleared => {

                    let current_time = self.timer.elapsed().as_millis();
                    if current_time - prev_time >= 100 {
                        prev_time = current_time;
                        fps = counter * 10;
                        match self.initial_time_step {
                            Some(_) => {
                                self.time_step = 1_f64 / (fps as f64);
                            },
                            None => ()
                        }
                        counter = 0;
                    }
                    
                    unsafe {
                        // Update to some color
                        // Clear Screen
                        gl::ClearColor(0.45, 0.45, 0.45, 0.8);
                        gl::Clear(gl::COLOR_BUFFER_BIT);
                        gl::Clear(gl::DEPTH_BUFFER_BIT);
                    }

                    match self.solver {
                        
                        Solver::None => {},
                        _ => {

                            solution = match solver.solve(self.time_step) {
                                Ok(solution) => solution,
                                Err(e) => panic!("Error while solving equation!: {}",e)
                            };
                
                            // updating colors. One time per vertex should be updated (that is, every 6 steps).
                            self.mesh.update_gradient_1d(solution.iter().map(|x| x.abs()).collect());
                            
                            if let Err(e) = self.mesh.bind_all_no_texture() {
                                panic!("Error while binding mesh again!: {}",e)
                            }
                            if let Err(e) = self.mesh.send_to_gpu() {
                                panic!("Error while sending updated mesh to GPU!: {}",e)
                            }
                        
                        }

                    }
        
        
                    
                    // Text shader to draw text
                    self.text_shader.use_shader();
        
                    if let Err(e) = self.character_set.bind_all() {
                        panic!("Error while binding character set again! {}",e)
                    }
                    if let Err(e) = self.character_set.draw_text(format!(
                        "x: {}, y: {}, FPS: {}",
                        self.mouse_coordinates.x, self.mouse_coordinates.y, fps
                    )) {
                        panic!("Error while writing coordinates and fps counter: {}",e);
                    }

                    if let Err(e) = self.character_set.unbind_texture() {
                        panic!("Error while unbinding texture for character set!: {}",e)
                    }
        
                    // Geometry shader to draw mesh
                    self.geometry_shader.use_shader();
                    if let Err(e) = self.geometry_shader
                        .set_mat4("view", &self.camera.view_matrix) {
                            panic!("Unable to set new view matrix for geometry!: {}",e)
                        }
        
                    if let Err(e) = self.mesh.bind_vao() {
                        panic!("Unable to bind vao of mesh!: {}",e)
                    }

                    // Draw filled or not filled
                    match fill {
                        false => {}
                        true => unsafe {
                            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                        }
                    }
                    if let Err(e) = self.mesh.draw() {
                        panic!("Unable to draw mesh!: {e}")
                    }
                    // Need to change old and new buffer to redraw
                    if let Err(e) = self.context.swap_buffers() {
                        panic!("Unable to swap buffers!: {}",e)
                    }
                    counter += 1;
                },

                _ => (),
            }
        })
    }
}
