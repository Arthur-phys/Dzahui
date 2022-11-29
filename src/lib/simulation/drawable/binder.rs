// Internal dependencies
use crate::Error;

// External dependencies
use gl::{ self, types::{GLfloat, GLsizei, GLsizeiptr, GLuint}};
use std::{mem, os::raw::c_void, ptr};
use ndarray::Array1;


/// # General Information
///
/// An object that can be represented in CPU via a, ebo, vbo, vao and texture (the latter is not necessary).
///
pub(crate) trait Bindable {
    /// Obtains binder associated to object. Getter.
    fn get_binder(&self) -> Result<&Binder, Error>;

    /// Obtain binder mutable reference.
    fn get_mut_binder(&mut self) -> Result<&mut Binder, Error>;

    /// Shortcut to binder setup function.
    fn setup(&mut self) -> Result<(), Error> {
        Ok(self.get_mut_binder()?.setup())
    }

    /// Shortcut to setup texture function.
    fn setup_texture(&mut self) -> Result<(), Error> {
        Ok(self.get_mut_binder()?.setup_texture())
    }

    /// Shortcut to bind all without texture function.
    fn bind_all_no_texture(&self) -> Result<(), Error> {
        Ok(self.get_binder()?.bind_all_no_texture())
    }

    /// Shortcut to bind all function.
    fn bind_all(&self) -> Result<(), Error> {
        Ok(self.get_binder()?.bind_all())
    }

    /// Shortcut to bind vao function
    fn bind_vao(&self) -> Result<(), Error> {
        Ok(self.get_binder()?.bind_vao())
    }

    /// Shortcut to bind texture
    fn bind_texture(&self) -> Result<(), Error> {
        Ok(self.get_binder()?.bind_texture())
    }

    /// Shortcut to unbind texture
    fn unbind_texture(&self) -> Result<(), Error> {
        Ok(self.get_binder()?.unbind_texture())
    }
}

/// # General Information
///
/// All objects that can be drawn by OpenGL should implement a drawable trait. The main functions are
/// setup and draw. Both which contain general implementations to setup drawable object in GPU and draw it respectively.
///
pub(crate) trait Drawable: Bindable {
    /// Creates a way to obtain vertices from drawable object. Getter.
    fn get_vertices(&self) -> Result<Array1<f32>, Error>;
    /// Creates a way to obtain indices to draw vertices (and triangles). Getter.
    fn get_indices(&self) -> Result<&Array1<u32>, Error>;
    /// Creates a way to obtain order of object's dimensions. Getter.
    fn get_max_length(&self) -> Result<f32, Error>;

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
    fn send_to_gpu(&self) -> Result<(), Error> {
        let vertices = self.get_vertices()?;
        let indices = self.get_indices()?;

        unsafe {
            // Point to data, specify data length and how it should be drawn (static draw serves to only draw once).
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                // Double casting to raw pointer. Equivalent to C's void type when used as pointer.
                gl::DYNAMIC_DRAW,
            );

            // Point to data, specify data length and how it should be drawn
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
                &indices[0] as *const u32 as *const c_void,
                gl::DYNAMIC_DRAW,
            );

            // How should coordinates be read.
            // Reading starts at index 0.
            // Each coordinate is composed of 3 values.
            // No normalized coordinates.
            // The next coordinate is located 3 values after the first index of the previous one.
            // The offset to start reading coordinates (for position it's normally zero. It is used when having texture and/or color coordinates).
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (6 * mem::size_of::<GLfloat>()) as GLsizei,
                ptr::null(),
            );
            // Enable vertex atributes giving vertex location (setup in vertex shader).
            gl::EnableVertexAttribArray(0);

            // Enable color visibility
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                (6 * mem::size_of::<GLfloat>()) as GLsizei,
                (3 * mem::size_of::<GLfloat>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);
        }
        Ok(())
    }

    /// # General Information
    ///
    /// A simple call to glDrawElements in triangles mode. It assumes all information to be drawn has been sent and is stored in a single vbo, veo pair.
    /// (Making multiple calls to draw is, in general, not a good idea, since it can really slow down a program reducing the FPS. When drawing
    /// multiple objects, it's better to use the so called 'batch rendering').
    ///
    /// # Parameters
    ///
    /// * `&self` - A reference to the object which is attached to a binder and knows how to get the indices and indices length.
    ///
    fn draw(&self) -> Result<(), Error> {
        let indices_len: i32 = self.get_indices()?.len() as i32;

        // Draw only when window is created and inside loop
        // Drawn as triangles
        unsafe {
            // Draw
            // Comment to see the triangles filled instead of only the lines that form them.
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            gl::DrawElements(gl::TRIANGLES, indices_len, gl::UNSIGNED_INT, ptr::null());
        }

        Ok(())
    }
}

/// # General Information
///
/// Variables asocciated with GPU and drawable object(s). Assigned by OpenGL. Should always be mutable.
///
/// # Fields
///
/// * `vbo` (Vertex Buffer Object) -  Vertices Generated by Mesh.
/// * `vao` (Vertex Array Object) - Binds vertices and it's configuration with OpenGL.
/// * `ebo` (Element Buffer Object) - Indices to draw vertices.
/// * `texture` - Texture in 2D to use over object to draw.
///
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Binder {
    pub(crate) vbo: u32,
    pub(crate) vao: u32,
    pub(crate) ebo: u32,
    pub(crate) texture: u32,
}

impl Binder {
    /// Simple new function. Generates new instance of Binder.
    pub(crate) fn new() -> Binder {
        Binder {
            vbo: 0,
            vao: 0,
            ebo: 0,
            texture: 0,
        }
    }

    /// # General Information
    ///
    /// sets up binder's variables with GPU. Should always be used after instance of window has set up OpenGL context. Never binds texture.
    ///
    /// # Parameters
    ///
    /// * `&mut self` - OpenGL changes the values of instance fields effectively setting up linkage beetween vao and vbo and ebo. Texture has to be set up later since not
    /// all drawings have it.
    ///
    pub(crate) fn setup(&mut self) {
        unsafe {
            // Create VAO
            gl::GenVertexArrays(1, &mut self.vao);
            // Bind Vertex Array Object first
            // Since it is bound first, it binds to the EBO and VBO (because they are the only ones being bound after it)
            gl::BindVertexArray(self.vao);

            // Generates a VBO in GPU
            gl::GenBuffers(1, &mut self.vbo);
            // Generates a EBO in GPU
            gl::GenBuffers(1, &mut self.ebo);
            // Bind VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            // BInd VAO
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
        }
    }

    /// # General Information
    ///
    /// Binds binder's vao to use.
    ///
    /// # Parameters
    ///
    /// * `&self` - Instance does not need to be mutable since it's already setup.
    ///
    pub(crate) fn bind_vao(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    /// # General Information
    ///
    /// Binds binder's vbo to use.
    ///
    /// # Parameters
    ///
    /// * `&self` - Instance does not need to be mutable since it's already setup.
    ///
    pub(crate) fn bind_vbo(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        }
    }

    /// # General Information
    ///
    /// Binds binder's ebo to use.
    ///
    /// # Parameters
    ///
    /// * `&self` - Instance does not need to be mutable since it's already setup.
    ///
    pub(crate) fn bind_ebo(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
        }
    }

    /// # General Information
    ///
    /// Binds vao, ebo and vbo. Never binds texture.
    ///
    /// # Parameters
    ///
    /// * `&self` - Instance does not need to be mutable since it's already setup.
    ///
    #[allow(dead_code)]
    pub(crate) fn bind_all_no_texture(&self) {
        self.bind_vao();
        self.bind_vbo();
        self.bind_ebo();
    }

    /// # General Information
    ///
    /// Binds a 2D texture providing a standar way to scale up and down (mipmap), and enabling blending so that alpha channel is respected.
    /// When enabling blending, it's necessary to change how it occurs: If alpha channel is to be respected, the incoming pixels have to be alpha transparent,
    /// while the pixes already present have to be (1-alpha) transparent.
    ///
    /// # Parameters
    ///
    /// * `&mut self` - OpenGL changes values of instance field texture effectively linking current texture to use.
    ///
    pub(crate) fn setup_texture(&mut self) {
        unsafe {
            // generate and bind texture
            gl::GenTextures(1, &mut self.texture);
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
        }
    }

    /// # General Information
    ///
    /// Binds binder's texture to use.
    ///
    /// # Parameters
    ///
    /// * `&self` - Instance does not need to be mutable since it's already setup.
    ///
    pub(crate) fn bind_texture(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
        }
    }

    /// # General information
    ///
    /// Unbind texture when needed. Since not all objects posess a texture, this has to be done sometimes.
    ///
    pub(crate) fn unbind_texture(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    /// # General Information
    ///
    /// Binds vao, ebo, vbo and texture.
    ///
    /// # Parameters
    ///
    /// * `&self` - Instance does not need to be mutable since it's already setup.
    ///
    pub(crate) fn bind_all(&self) {
        self.bind_vao();
        self.bind_vbo();
        self.bind_ebo();
        self.bind_texture();
    }
}
