use std::{fs::File,collections::HashMap,io::{BufReader, BufRead},ptr,mem,os::raw::c_void};
use gl::{self,types::{GLsizei, GLsizeiptr, GLuint, GLfloat}};
use cgmath::{Vector4,Matrix4,Transform,Vector3};
use image;

use crate::Error;

use super::{binder::Binder, Bindable};
use super::super::camera::Camera;

/// # General Information
/// 
/// Representation of a character
/// 
/// # Fields
/// 
/// * `id` - Id of a character (unicode).
/// * `origin` - Place of the character in charmap image. 
/// * `size` - Width and height in a tuple. 
/// * `character_start` - Where the character begins from rectangle ditctated in size. 
/// 
#[derive(Debug)]
struct Character {
    // Character id
    pub id: u32,
    // Where it starts (top left corner)
    pub(crate) origin: (f32,f32),
    // Width and height of texture representing character
    pub(crate) size: (f32,f32),
    // Offset from top left corner
    pub(crate) character_start: (f32,f32),
}

/// # General Information
/// 
/// A list of characters with a series of important options that make it into a font.
/// 
/// # Fields
/// 
/// * `characters` - Every character literal (char) with it's corresponding character struct.
/// * `font_type` - Name of the font.
/// * `font_size` - Size of the font (pt).
/// * `is_italic` - Self-explanatory.
/// * `is_bool` - Self-explanatory.
/// * `encoding` - Type of encoding (unicode, normally)
/// * `line_height` - Where characters should start to be drawn vertically.
/// * `character_number` - Number of characters in font.
/// * `texture_file` - Where texture file is located.
/// * `texture_size` - Size of the whole texture file.
/// * `binder` - Binder associated to font.
/// * `image_as_vec` - Image as a vector.
/// 
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct CharacterSet {
    characters: HashMap<char, Character>,
    font_type: String,
    font_size: u32, // pt
    is_italic: bool,
    is_bold: bool,
    encoding: String,
    line_height: u32, // Pixels
    character_number: usize,
    texture_file: String,
    texture_size: (u32,u32), // Pixels
    pub(crate) binder: Binder,
    image_as_vec: Vec<u8>, // image vector
}

impl Character {

    /// New instance of a character
    pub fn new(id: u32, origin: (f32,f32), size: (f32,f32), character_start: (f32,f32)) -> Self {
        Self { 
            id,
            origin,
            size,
            character_start,
        }
    }

}

impl PartialEq for Character {
    
    fn eq(&self, other: &Self) -> bool {
        if self.id == other.id {
            true
        } else {
            false
        }
    }

}

impl Eq for Character {}

impl Bindable for CharacterSet {
    
    fn get_binder(&self) -> Result<&Binder,Error> {
        Ok(&self.binder)
    }

    fn get_mut_binder(&mut self) -> Result<&mut Binder,Error> {
        Ok(&mut self.binder)
    }
}

impl CharacterSet {

    /// # General Information
    /// 
    /// New character set given a character file. It reads every line, substracting information neccesary to get every struct property and later
    /// create every character and associating it to it's char. It also loads the image and generates a vector with it within.
    /// 
    /// # Parameters
    /// 
    /// * `character_file` - fnt file. It is important that the file is correctly created since metadata is important to struct instance.
    /// 
    pub fn new(character_file: &str) -> Self {

        let binder = Binder::new();
        
        let file = File::open(character_file).expect("Unable to open file. Does the file exists and is readable?");
        let mut reader = BufReader::new(file).lines();

        // read general properties of font first
        let info_line = reader.next().unwrap().expect("Unable to read first line of file propperly.");
        let info_line: Vec<&str> = info_line.split("\"").collect();

        // Font properties
        let font_type = info_line[1].to_string();

        // Need to split againd but this time via space, collecting every property from first line
        let mut property_map_one: HashMap<String,String> = info_line[2].trim().split(" ").map(|property| {
            let key_value: Vec<&str> = property.split("=").collect();
            (key_value[0].to_string(),key_value[1].to_string())
        }).collect();
        
        // Second line also contains information
        let second_info_line = reader.next().unwrap().expect("Unable to read second line of file propperly.");
        let mut property_map_two = second_info_line.split(" ");
        // Skip 'common' word
        property_map_two.next();
        // Shadowing
        let mut property_map_two: HashMap<String,String> = property_map_two.map(|property | {
            let key_value: Vec<&str> = property.split("=").collect();
            (key_value[0].to_string(),key_value[1].to_string())
        }).collect();

        // Third line contains texture file
        let third_info_line = reader.next().unwrap().expect("Unable to read third line of file propperly.");
        let mut property_map_three = third_info_line.split(" ");
        // SKip 'page' word
        property_map_three.next();
        // Shadowing
        let mut property_map_three: HashMap<String,String> = property_map_three.map(|property | {
            let key_value: Vec<&str> = property.split("=").collect();
            (key_value[0].to_string(),key_value[1].to_string())
        }).collect();

        // After third line, image can be loaded.
        let img = image::open(format!("./assets/{}", property_map_three.get("file").expect("Font file not found.").replace("\"",""))).unwrap();
        let img_vec: Vec<u8> = img.into_bytes();

        // Fourth line contains number of characters
        let fourth_info_line = reader.next().unwrap().expect("Unable to read fourth line of file propperly.");
        let mut property_map_four = fourth_info_line.split(" ");
        // SKip 'chars' word
        property_map_four.next();
        // Shadowing
        let mut property_map_four: HashMap<String,usize> = property_map_four.map(|property | {
            let key_value: Vec<&str> = property.split("=").collect();
            (key_value[0].to_string(),key_value[1].parse().unwrap())
        }).collect();
        

        // Processing rest of file to create the characters
        let mut characters: HashMap<char,Character> = HashMap::with_capacity(*property_map_four.get("count").unwrap());
        for line in reader {
            match line {
                Ok(content) => {
                    
                    // Get rid of multiple space
                    let mut properties = content.split(" ").filter(|e| {*e != ""});
                    let is_character_line = properties.next().unwrap();
                    
                    if is_character_line != "char" {
                        continue;
                    
                    } else {

                        // Property map for character
                        let property_map: HashMap<&str,f32> = properties.map(|property| {
                            let key_value: Vec<&str> = property.split("=").collect();
                            (key_value[0],key_value[1].parse().unwrap())
                        }).collect();

                        // Character creation

                        let temp_character = Character::new(property_map["id"] as u32,
                        (property_map["x"],property_map["y"]),
                        (property_map["width"],property_map["height"]),
                        (property_map["xoffset"],property_map["yoffset"]),
                        );
                        
                        // Insert character
                        characters.insert(char::from_u32(property_map["id"] as u32).unwrap(), temp_character);
                    }
                },
                Err(e) => panic!("Unable to read file propperly: {}",e)
            }
        }
        
        Self {
            font_type,
            characters,
            binder,
            image_as_vec: img_vec,
            font_size: property_map_one.remove("size").expect("Size parameter not found").parse().unwrap(),
            is_bold: property_map_one.get("bold").expect("Bold parameter not found") == "1",
            is_italic: property_map_one.get("bold").expect("Bold parameter not found") == "1",
            encoding: String::from("unicode"),
            line_height: property_map_two.remove("lineHeight").expect("Line height property not found").parse().unwrap(),
            texture_size: (
                property_map_two.remove("scaleW").expect("Width property not found").parse().unwrap(),
                property_map_two.remove("scaleH").expect("Height property not found").parse().unwrap(),
            ),
            texture_file: property_map_three.remove("file").expect("Font file not found.").replace("\"",""),
            character_number: property_map_four.remove("count").expect("Character count not found")
         }
    }

    /// # General Information
    /// 
    /// Struct has it's own method to send to gpu since texture has to be considered. This means send_to_gpu method inside bindable trait does not work 
    /// with this struct. Text are sent by letter. Performance is not affected much since the amount of letters is small. Image vector is sent in it's entirety.
    /// 
    /// # Parameters
    /// 
    /// * `&self` - Only a couple properties within self are enough to configure: vector from image, size of image.
    /// 
    pub(crate) fn send_to_gpu(&self) {

        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA,gl::ONE_MINUS_SRC_ALPHA);

            // texture wrapping parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); //how to wrap in s coordinate
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32); // how to wrap in t coordinate
            // texture filtering
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32); // when texture is small, scall using linear
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32); // when texture is big, scall using linear

            gl::TexImage2D(gl::TEXTURE_2D, // Texture target is 2D since we created a texture for that
                0, // Mipmap level 0 which is default. Otherwise wue could specify levels and change it
                gl::RGBA as i32, // Image is given as values of RGB
                self.texture_size.0 as i32,
                self.texture_size.1 as i32,
                0, // Legacy sutff not explained
                gl::RGBA, // Format of the image (this is the actual format)
                gl::UNSIGNED_BYTE, // RGB values are given as chars
                &self.image_as_vec[0] as *const u8 as *const c_void); // Pointer to first element of vector

            gl::GenerateMipmap(gl::TEXTURE_2D); // generate mipmap for texture 2d (when object is far or close)

            // set up way information will be sent
            // vertex coordinates
            gl::VertexAttribPointer(0,3,gl::FLOAT,gl::FALSE,5*mem::size_of::<GLfloat>() as GLsizei, ptr::null());
            gl::EnableVertexAttribArray(0); // Enabling vertex atributes giving vertex location (setup in vertex shader).
            // texture coordinates
            gl::VertexAttribPointer(1,2,gl::FLOAT,gl::FALSE,5*mem::size_of::<GLfloat>() as GLsizei, (3 * mem::size_of::<GLfloat>()) as *const c_void);
            gl::EnableVertexAttribArray(1); // Enabling vertex atributes giving vertex location (setup in vertex shader).

            // now allocate quad (two triangles) information to be sent
            gl::BufferData(gl::ARRAY_BUFFER,
                (5 * 4 * mem::size_of::<GLfloat>()) as GLsizeiptr, // number of vertices * number of values in each * size of float 32 bits
                ptr::null() as *const f32 as *const c_void,
                gl::DYNAMIC_DRAW); // dynamic draw since content will be altered constantly
    
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (6 * mem::size_of::<GLfloat>()) as GLsizeiptr,
                ptr::null() as *const u32 as *const c_void, 
                gl::DYNAMIC_DRAW);
            
            }
    }

    /// # General Information
    /// 
    /// Obtain every letter given an entry of text, generating everything necessary to send each one to the gpu: indices, texture coordinates.
    /// 
    /// # Parameters
    /// 
    /// * `&self` - Obtain character struct for a given character to access it's properties.
    /// * `text` - A text string to parse and display on screen. Every character has to be in the original font (CharacterSet).
    /// 
    fn get_vertices_from_text<A: AsRef<str>>(&self, text: A) -> (Vec<[f32;20]>,Vec<[u32;6]>) {

        // Split text into chars. Should be feasible given the fact that we only operate with the alphabet, numbers and some special symbols such as '?','!'
        // Range of utf-8 values: 0,2^21 (given that there are at most 11 bits of metadata in a 4 byte sequence)
        let text_vec: Vec<char> = text.as_ref().chars().collect();
        // Initialize vertices and indices vectors
        let mut vertices: Vec<[f32;20]> = Vec::new();
        let mut indices: Vec<[u32;6]> = Vec::new();
        
        // Obtain subset of characters from CharacterSet HashMap
        text_vec.iter().fold((0.0_f32, 0_u32),|(width,last_index), character_string| {
            
            let character_struct = self.characters.get(character_string);
            match character_struct {
                
                Some(character) => {
                    
                    // vertices obtained from character
                    let width = width + character.size.0;
                    let height = character.size.1;
                    // Point order:

                    //   start ---->
                    //   ^         |
                    //   |         |
                    //   |         |
                    //   |         ˇ
                    //    <--------

                    let new_vertices: [f32; 20] = [
                        // First point
                        // Coordinate
                        width - character.size.0,
                        height,
                        0.0,
                        // Texture
                        (character.origin.0)/(self.texture_size.0 as f32),
                        (character.origin.1)/(self.texture_size.1 as f32),
                        // Second point
                        // Coordinate
                        width,
                        height,
                        0.0,
                        // Texture
                        (character.origin.0 + character.size.0)/(self.texture_size.0 as f32),
                        (character.origin.1)/(self.texture_size.1 as f32),
                        // Third point
                        // Coordinate
                        width,
                        0.0,
                        0.0,
                        // Texture
                        (character.origin.0 + character.size.0)/(self.texture_size.0 as f32),
                        (character.origin.1 + character.size.1)/(self.texture_size.1 as f32),
                        // Fourth point
                        // Coordinate
                        width - character.size.0,
                        0.0, // y always starts on 0.0
                        0.0, // z will always be 0.0 initially
                        // Texture
                        (character.origin.0)/(self.texture_size.0 as f32),
                        (character.origin.1 + character.size.1)/(self.texture_size.1 as f32),
                    ];
                    let new_indices: [u32; 6] = [
                        // First index is the one passed from last iteration.
                        // There are six indices total
                        // First triangle
                        0, 1, 2,
                        // Second triangle
                        2, 3, 0
                    ];

                    vertices.push(new_vertices);
                    indices.push(new_indices);
                    
                    (width,last_index + 4)
                },
                None => panic!("Character {} does not exist on CharacterSet",character_string)
            }
        });
        (vertices,indices)
    }

    /// # General Information
    /// 
    /// Draw a given text string. It can even be dynamic and, as long as the text is not too big, there will be no framerate drop.
    /// 
    /// # Parameters
    /// 
    /// * `&self` - Obtain vertices from text function
    /// * `text` - A given text input to draw into screen
    /// 
    pub(crate) fn draw_text<A: AsRef<str>>(&self, text: A) {

        // use function inside event loop in dzahui window, not anywhere else.
        // obtain vertices and indices to draw
        let (vertices, indices) = self.get_vertices_from_text(text);

        vertices.iter().zip(indices).for_each(|(vertices_subset, indices_subset)| {
            unsafe {

                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);

                gl::BufferSubData(gl::ARRAY_BUFFER, 0,
                    (vertices_subset.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                    &vertices_subset[0] as *const f32 as *const c_void); // double casting to raw pointer of c_void
    
                gl::BufferSubData(gl::ELEMENT_ARRAY_BUFFER, 0, (indices_subset.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
                    &indices_subset[0] as *const u32 as *const c_void);
                    
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
                
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

            }
        });
    }

    /// # General Information
    /// 
    /// Obtain a matrix to make text appear on screen in a certain position with a certain size. There's still a problem of scale happening: it is hardcoded.
    /// 
    /// # Parameters
    /// 
    /// * `viewport_x` - Place where text will be rendered in viewport.
    /// * `viewport_y` - Place where text will be rendered in viewport.
    /// * `camera` - A reference to a camera to obtain a projection matrix (should change)
    /// * `window_height` - Size of viewport to normalize coordinates.
    /// * `window_width` - Size of viewport to normalize coordinates.
    ///
    pub(crate) fn matrix_for_screen(viewport_x: f32, viewport_y: f32, camera: &Camera, window_height: u32, window_width: u32) -> Matrix4<f32> {
        
        let ndc_coordinates = Vector4::new(
            (viewport_x - (window_width as f32)/2.0)/((window_width as f32)/2.0), // map between -1 and 1
            (viewport_y - (window_height as f32)/2.0)/((window_height as f32)/2.0),
            -0.5, // between near and mesh (-1.0,0.0)
            1.0
        );
        
        let inverse_projection_matrix: Matrix4<f32> = camera.projection_matrix.inverse_transform().expect("No inverse transform exists for this matrix");
        let view_coordinates = inverse_projection_matrix * ndc_coordinates;
        
        // need to divide by w (god knows why)
        let view_coordinates = Vector3::new(view_coordinates.x,view_coordinates.y,view_coordinates.z) / view_coordinates.w;

        Matrix4::from_translation(view_coordinates) * Matrix4::from_scale(0.0001)

    }

}

// #[cfg(test)]
// mod test {
//     use std::collections::HashMap;

//     use super::{CharacterSet, Character};
    
//     #[test]
//     fn read_properly() {
//         let set = CharacterSet::new("./assets/dzahui-font_test.fnt");
//         let should_be_set = CharacterSet {
//             characters: HashMap::from([
//                 ('{', Character::new(123,(0.0,0.0),(21.0,61.0),(1.0,13.0))),
//                 (' ',Character::new(32, (0.0,0.0), (0.0,0.0), (5.0,18.0))),
//                 ('a',Character::new(97, (211.0,153.0), (35.0,37.0), (2.0,25.0))),
//             ]),
//             font_type: "Liberation Sans".to_string(),
//             font_size: 12,
//             is_italic: false,
//             is_bold: false,
//             line_height: 19,
//             encoding: "unicode".to_string(),
//             texture_file: "dzahui-font.png".to_string(),
//             texture_size: (640, 394),
//             character_number: 3,
//         };
//         assert!( set == should_be_set );
//     }

//     #[test]
//     fn test_vertices_content() {
//         let set = CharacterSet::new("./assets/dzahui-font_test.fnt");
//         let (vertices, indices) = set.get_vertices_from_text("{a{{{a");
//         // number os squares (quads) should be 6, equal to the number of chars in text
//         assert!( indices.len()/6 == 6 );
//         assert!( vertices.len()/20 == 6 );
//     }
// }