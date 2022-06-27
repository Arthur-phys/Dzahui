use std::{ffi::CString,ptr,fs::File};
use cgmath::{Matrix4,Matrix};
use gl::types::{GLint};
use std::io::Read;
use gl;

pub struct Shader {
    pub id: u32,
}

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Self {
        // opening files
        let mut vertex_shader = File::open(vertex_path).expect("Unable to open the requested file for vertex shader.");
        let mut fragment_shader = File::open(fragment_path).expect("Unable to open the requested file for fragment shader.");
        // reading files
        let mut vertex_shader_read = String::new();
        let mut fragment_shader_read = String::new();
        vertex_shader.read_to_string(&mut vertex_shader_read).expect("Unable to read file for vertex shader.");
        fragment_shader.read_to_string(&mut fragment_shader_read).expect("Unable to read file for fragment shader");
        // compiling files
        let vertex_shader_read = CString::new(vertex_shader_read.as_bytes()).unwrap();
        let fragment_shader_read = CString::new(fragment_shader_read.as_bytes()).unwrap();
        // creating logs
        // compiling vertex shader

        let vertex_shader: u32;
        unsafe {
            vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex_shader,1,&vertex_shader_read.as_ptr(),ptr::null());
            gl::CompileShader(vertex_shader);
            let mut success = gl::FALSE as GLint;
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success == gl::FALSE as GLint { // log does not serve
                println!("Error while compiling vertex shader!");
            }
        };
        // compiling fragment shader
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
        // final part
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

        Shader { id }
    }
    
    pub fn use_shader(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_bool(&self, opengl_variable_name: &str, bool_value: bool) {
        let c_str_name = CString::new(opengl_variable_name.as_bytes()).unwrap();
        unsafe {
           gl::Uniform1i(gl::GetUniformLocation(self.id, c_str_name.as_ptr()),bool_value as i32);
        }
    }

    pub fn set_int(&self, opengl_variable_name: &str, int_value: i32) {
        let c_str_name = CString::new(opengl_variable_name.as_bytes()).unwrap();
        unsafe {
           gl::Uniform1i(gl::GetUniformLocation(self.id, c_str_name.as_ptr()),int_value);
        }
    }

    pub fn set_float(&self, opengl_variable_name: &str, float_value: f32) {
        let c_str_name = CString::new(opengl_variable_name.as_bytes()).unwrap();
        unsafe {
           gl::Uniform1f(gl::GetUniformLocation(self.id, c_str_name.as_ptr()),float_value);
        }
    }

    pub fn set_mat4(&self, opengl_variable_name: &str, mat4_value: &Matrix4<f32>) {
        let c_str_name = CString::new(opengl_variable_name.as_bytes()).unwrap();
        unsafe {
            gl::UniformMatrix4fv(gl::GetUniformLocation(self.id, c_str_name.as_ptr()),1,gl::FALSE,mat4_value.as_ptr());
        }
    }
}