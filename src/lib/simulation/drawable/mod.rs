/// Common functions in drawable (2D or 3D objects)
pub(crate) mod text;
pub(crate) mod binder;

use std::{ptr,mem,os::raw::c_void};
use gl::{self,types::{GLsizei, GLsizeiptr, GLuint, GLfloat}};
use ndarray::Array1;

use crate::{DzahuiWindow, Error};
use binder::Binder;

/// General Information
/// 
/// An object that can be represented in CPU via a, ebo, vbo, vao and texture (the latter is not necessary).
/// 
pub(crate) trait Bindable {

        /// Obtains binder associated to object. Getter.
        fn get_binder(&self) -> &Binder;
        
        /// Obtain binder mutable reference.
        fn get_mut_binder(&mut self) -> &mut Binder;
        
        /// Shortcut to binder setup function.
        fn setup(&mut self) {
            self.get_mut_binder().setup()
        }

        /// Shortcut to setup texture function.
        fn setup_texture(&mut self) {
            self.get_mut_binder().setup_texture()
        }

        /// Shortcut to bind all without texture function.
        fn bind_all_no_texture(&self) {
            self.get_binder().bind_all_no_texture()
        }

        /// Shortcut to bind all function.
        fn bind_all(&self) {
            self.get_binder().bind_all()
        }

        /// Shortcut to bind vao function
        fn bind_vao(&self) {
            self.get_binder().bind_vao()
        }

        /// Shortcut to bind texture
        fn bind_texture(&self) {
            self.get_binder().bind_texture()
        }

        /// Shortcut to unbind texture
        fn unbind_texture(&self) {
            self.get_binder().unbind_texture()
        }

}


/// # General Information
/// 
/// All objects that can be drawn by OpenGL should implement a drawable trait. The main functions are
/// setup and draw. Both which contain general implementations to setup drawable object in GPU and draw it respectively.
///
pub(crate) trait Drawable: Bindable {
     
    /// Creates a way to obtain vertices from drawable object. Getter.
    fn get_vertices(&self) -> Array1<f32>;
    /// Creates a way to obtain indices to draw vertices (and triangles). Getter.
    fn get_triangles(&self) -> &Array1<u32>;
    /// Creates a way to obtain order of object's dimensions. Getter.
    fn get_max_length(&self) -> Result<f32,Error>;

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

    /// # General Information
    /// 
    /// A simple call to glDrawElements in triangles mode. It assumes all information to be drawn has been sent and is stored in a single vbo, veo pair.
    /// This means this functions sometimes will be overriden by classes' behavior or even not implemented when objects need a more versatile version of the
    /// function (Making multiple calls to draw is, in general, not a good idea, since it can really slow down a program reducing the FPS. When drawing
    /// multiple objects, it'ss better to use the so called 'batch rendering').
    /// 
    /// # Parameters
    /// 
    /// * `&self` - A reference to the object which is attached to a binder and knows how to get the indices and indices length.
    /// 
    fn draw(&self, window: &DzahuiWindow) {

        let indices_len: i32 = self.get_triangles().len() as i32;
        
        // Draw only when window is created and inside loop
        // Drawn as triangles
        unsafe {
            // Draw
            gl::DrawElements(gl::TRIANGLES,indices_len,gl::UNSIGNED_INT,ptr::null());
        }
    }
}

pub(crate) trait TextureDrawable: Bindable {

}