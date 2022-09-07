use std::{ffi::CString,ptr,fs::File};
use cgmath::{Matrix4,Matrix};
use gl::types::{GLint};
use std::io::Read;
use gl;
use crate::Error;

/// # General information
/// 
/// A shader instance stores the id representing a combination of a vertex and fragment shader compiled (OpenGL Shading Language) 
/// and attached to an OpenGL program. Later on, an instance of such shader can be attached to an OpenGL context to use in conjunction with a series
/// of vertices to draw to screen.
/// 
/// # Fields
/// 
/// * `id` - An id field setup by OpenGL to uniquely identify shaders being passed.
/// 
#[derive(Debug, PartialEq, Eq)]
pub struct Shader {
    pub(crate) id: u32,
}

// Added because not all functions to send variables to vertex shader are used but still want to provide functionality to do it.
#[allow(dead_code)]
impl Shader {
    /// # General information
    /// 
    /// Creates a new shader program composed of both a vertex and a frament shader. Since it uses `gl` crate, it's necessary that an openGL context has
    /// been initialized. Unsafe part stops rust from caching errors while compiling shaders, therefore print statements will be sent to the terminal containing
    /// a message in case an error has happened. Later use of faulty shaders will stop the program from running, but debbuging becomes hard since little
    /// information is provided by the `gl` crate. Should enable logging errors at a later date.
    /// **Regarding the steps the function uses**: first it opens and read files to strings. Then, shaders are casted to CStrings. After that, each shader is sent
    /// to be compiled and linked to a u32 variable. Finally, the u32 varaibles are linked to an OpenGL program with an id and cache is erased (compiled programs
    /// are already associated to a program, therefore can be safely erased). This last id is returned inside Shader structure.
    /// 
    /// # Parameters
    /// 
    /// * `vertex_path` - Path to a vertex shader file.
    /// * `fragment_path` - Path to a fragment shader file.
    /// 
    pub fn new(vertex_path: impl AsRef<str>, fragment_path: impl AsRef<str>) -> Result<Self, Error> {
        // Opening files.
        let mut vertex_shader = File::open(vertex_path.as_ref()).map_err(|e| Error::Io(e))?;
        let mut fragment_shader = File::open(fragment_path.as_ref()).map_err(|e| Error::Io(e))?;
        
        // Reading files.
        let mut vertex_shader_read = String::new();
        let mut fragment_shader_read = String::new();
        vertex_shader.read_to_string(&mut vertex_shader_read).map_err(|e| Error::Io(e))?;
        fragment_shader.read_to_string(&mut fragment_shader_read).map_err(|e| Error::Io(e))?;
        
        // Casting shaders.
        let vertex_shader_read = CString::new(vertex_shader_read.as_bytes()).unwrap();
        let fragment_shader_read = CString::new(fragment_shader_read.as_bytes()).unwrap();
        
        // Compiling shaders with GLSL.
        // Vertex shader.
        let vertex_shader: u32;
        unsafe {
            vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex_shader,1,&vertex_shader_read.as_ptr(),ptr::null());
            gl::CompileShader(vertex_shader);
            let mut success = gl::FALSE as GLint;
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success == gl::FALSE as GLint { // log does not serve
                return Err(Error::custom("Error while compiling vertex shader!"));
            }
        };
        // Fragment shader.
        let fragment_shader: u32;
        unsafe {
            fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment_shader,1,&fragment_shader_read.as_ptr(),ptr::null());
            gl::CompileShader(fragment_shader);
            let mut success = gl::FALSE as GLint;
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success == gl::FALSE as GLint { // log does not serve
                println!("Error while compiling fragment shader!");
            }
        }

        // Linkage to OpenGL program.
        let id: u32;
        unsafe {
            id = gl::CreateProgram();
            gl::AttachShader(id,vertex_shader);
            gl::AttachShader(id,fragment_shader);
            gl::LinkProgram(id);
            let mut success = gl::FALSE as GLint;
            gl::GetProgramiv(id,gl::LINK_STATUS,&mut success);
            if success == gl::FALSE as GLint {
                println!("Error while linking program shader!");
            }
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        };

        Ok(Shader { id })
    }
    
    /// Use a certain pair of shaders identified by id. Program can have multiple shaders at once, but only one can be used at a time.
    pub fn use_shader(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    /// Send a bool variable to vertex shader. bool variable has to be declared as a uniform in shader and it's name must be known for this to work.
    pub fn set_bool(&self, opengl_variable_name: &str, bool_value: bool) {
        let c_str_name = CString::new(opengl_variable_name.as_bytes()).unwrap();
        unsafe {
           gl::Uniform1i(gl::GetUniformLocation(self.id, c_str_name.as_ptr()),bool_value as i32);
        }
    }

    /// Send a i32 variable to vertex shader. i32 variable has to be declared as a uniform in shader and it's name must be known for this to work.
    pub fn set_int(&self, opengl_variable_name: &str, int_value: i32) {
        let c_str_name = CString::new(opengl_variable_name.as_bytes()).unwrap();
        unsafe {
           gl::Uniform1i(gl::GetUniformLocation(self.id, c_str_name.as_ptr()),int_value);
        }
    }

    /// Send a f32 variable to vertex shader. f32 variable has to be declared as a uniform in shader and it's name must be known for this to work.
    pub fn set_float(&self, opengl_variable_name: &str, float_value: f32) {
        let c_str_name = CString::new(opengl_variable_name.as_bytes()).unwrap();
        unsafe {
           gl::Uniform1f(gl::GetUniformLocation(self.id, c_str_name.as_ptr()),float_value);
        }
    }

    /// Send a 4x4 matrix variable to vertex shader. Matrix variable has to be declared as a uniform in shader and it's name must be known for this to work.
    pub fn set_mat4(&self, opengl_variable_name: &str, mat4_value: &Matrix4<f32>) {
        let c_str_name = CString::new(opengl_variable_name.as_bytes()).unwrap();
        unsafe {
            gl::UniformMatrix4fv(gl::GetUniformLocation(self.id, c_str_name.as_ptr()),1,gl::FALSE,mat4_value.as_ptr());
        }
    }
}