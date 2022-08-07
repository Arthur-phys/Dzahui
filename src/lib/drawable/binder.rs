use gl;

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
pub struct Binder {
    pub(crate) vbo: u32,
    pub(crate) vao: u32,
    pub(crate) ebo: u32,
    pub(crate) texture: u32,
}

impl Binder {

    /// Simple new function. Generates new instance of Binder.
    pub(crate) fn new() -> Binder {
        Binder { vbo: 0, vao: 0, ebo: 0, texture: 0 }
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
            gl::GenVertexArrays(1,&mut self.vao);
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
            gl::GenTextures(1,&mut self.texture);
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
            gl::BindTexture(gl::TEXTURE_2D,self.texture);
        }
    }

    /// # General information
    /// 
    /// Unbind texture when needed. Since not all objects posess a texture, this has to be done sometimes.
    /// 
    pub(crate) fn unbind_texture(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D,0);
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