/// Common functions in drawable (2D or 3D objects)
pub mod mesh;
pub mod text;
pub mod binder;
pub mod from_obj;

use std::{ptr,mem,os::raw::c_void};
use gl::{self,types::{GLsizei, GLsizeiptr, GLuint, GLfloat}};
use cgmath::{Matrix4,SquareMatrix};
use crate::DzahuiWindow;
use binder::Binder;


/// # General Information
/// 
/// All objects that can be drawn by OpenGL should implement a drawable trait. The main functions are
/// setup and draw. Both which contain general implementations to setup drawable object in GPU and draw it respectively.
///
pub trait Drawable {
     
    /// Creates a way to obtain vertices from drawable object. Getter.
    fn get_vertices(&self) -> &Vec<f32>;
    /// Creates a way to obtain indices to draw vertices (and triangles). Getter.
    fn get_triangles(&self) -> &Vec<u32>;
    /// Creates a way to obtain order of object's dimensions. Getter.
    fn get_max_length(&self) -> f32;
    /// Obtains binder associated to mesh. Getter.
    fn get_binder(&self) -> &Binder;

    /// # General Information
    /// 
    /// Once an object with Drawable trait has been created it can be sent to gpu.
    /// This function will send vertex and indices information to GPU to be drawn on screen.
    /// There's a couple of steps that should never be skipped:
    /// 
    /// - Object's binder has to have been initialized prior to this function call.
    /// - Always bind object's binder's vao and/or texture.
    /// - There's no need to bind ebo or vbo once vao is bound.
    /// 
    /// # Parameters
    /// 
    /// * `&self` - All information is stored inside the object an accesed through the getter methods above.
    /// 
    fn send_to_gpu(&self) {

        // MOST IMPORTANT CALL IN FUNCTION
        self.get_binder().bind_vao();
        self.get_binder().bind_ebo();
        self.get_binder().bind_vbo();
        // MOST IMPORTANT CALL IN FUNCTION

        let vertices = self.get_vertices();
        let triangles = self.get_triangles();

        unsafe {

            // Point to data, specify data length and how it should be drawn (static draw serves to only draw once).
            gl::BufferData(gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                // Double casting to raw pointer. Equivalent to C's void type when used as pointer.
                gl::STATIC_DRAW);
                
            // Point to data, specify data length and how it should be drawn
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                (triangles.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
                &triangles[0] as *const u32 as *const c_void,
                gl::STATIC_DRAW);
                
            // How should coordinates be read.
            // Reading starts at index 0.
            // Each coordinate is composed of 3 values.
            // No normalized coordinates.
            // The next coordinate is located 3 values after the first index of the previous one.
            // The offset to start reading coordinates (for position it's normally zero. It is used when having texture and/or color coordinates).
            gl::VertexAttribPointer(0,3,gl::FLOAT,
                gl::FALSE,
                (3*mem::size_of::<GLfloat>()) as GLsizei,
                ptr::null());
                        
            // Enable vertex atributes giving vertex location (setup in vertex shader).
            gl::EnableVertexAttribArray(0);
            // Comment to see the traingles filled instead of only the lines that form them.
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        }
    }

    fn draw(&self, window: &DzahuiWindow) {

        // MOST IMPORTANT CALL
        self.get_binder().bind_vao();
        // MOST IMPORTANT CALL

        let indices_len: i32 = self.get_triangles().len() as i32;
        // use mesh model matrix
        window.geometry_shader.set_mat4("model", &Matrix4::identity());
        
        // Draw only when window is created and inside loop
        // Drawn as triangles
        unsafe {
            // Draw
            gl::DrawElements(gl::TRIANGLES,indices_len,gl::UNSIGNED_INT,ptr::null());
        }
    }
}