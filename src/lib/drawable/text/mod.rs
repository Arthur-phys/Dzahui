use std::{fs::File,collections::HashMap,io::{BufReader, BufRead}};
use crate::{Binder, DzahuiWindow};

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

impl Character {
    pub fn new(id: u32, origin: (f32,f32), size: (f32,f32), character_start: (f32,f32)) -> Self {
        Self { id, origin, size, character_start }
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

#[derive(Debug)]
#[derive(PartialEq, Eq)]
struct CharacterSet {
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
}

impl CharacterSet {
    
    pub fn new(character_file: &str) -> Self {
        
        let file = File::open(character_file).expect("Unable to open file. Does the file exists and is readable?");
        let mut reader = BufReader::new(file).lines();

        // read general properties of font first
        let info_line = reader.next().unwrap().expect("Unable to read first line of file propperly.");
        let info_line: Vec<&str> = info_line.split("\"").collect();
        // Font properties
        let font_type = info_line[1].to_string();
        // Need to split againd but this time via space collecting every property from first line
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
                        (property_map["xoffset"],property_map["yoffset"])
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

    fn get_vertices_from_text<A: AsRef<str>>(&self, text: A) -> (Vec<f32>,Vec<u32>) {
        // Split text into chars. Should be feasible given the fact that we only operate with the alphabet, numbers and some special symbols such as '?','!'
        // Range of utf-8 values: 0,2^21 (given that there are at most 11 bits of metadata in a 4 byte sequence)
        let text_vec: Vec<char> = text.as_ref().chars().collect();
        // Initialize vertices and indices vectors
        let mut vertices: Vec<f32> = Vec::with_capacity(text_vec.len()* 4);
        let mut indices: Vec<u32> = Vec::with_capacity(text_vec.len() * 4);
        
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

                    let mut new_vertices: Vec<f32> = vec![
                        // First point
                        // Coordinate
                        width - character.size.0,
                        height,
                        0.0,
                        // Texture
                        (character.origin.0)/(self.texture_size.0 as f32),
                        1.0 - (character.origin.1)/(self.texture_size.1 as f32),
                        // Second point
                        // Coordinate
                        width,
                        height,
                        0.0,
                        // Texture
                        (character.origin.0 + character.size.0)/(self.texture_size.0 as f32),
                        1.0 - (character.origin.1)/(self.texture_size.1 as f32),
                        // Third point
                        // Coordinate
                        width,
                        0.0,
                        0.0,
                        // Texture
                        (character.origin.0 + character.size.0)/(self.texture_size.0 as f32),
                        1.0 - (character.origin.1 + character.size.1)/(self.texture_size.1 as f32),
                        // Fourth point
                        // Coordinate
                        width - character.size.0,
                        0.0, // y always starts on 0.0
                        0.0, // z will always be 0.0 initially
                        // Texture
                        (character.origin.0)/(self.texture_size.0 as f32),
                        1.0 - (character.origin.1 + character.size.1)/(self.texture_size.1 as f32),
                    ];
                    let mut new_indices: Vec<u32> = vec![
                        // First index is the one passed from last iteration.
                        // There are six indices total
                        // First triangle
                        last_index, last_index + 1, last_index + 2,
                        // Second triangle
                        last_index + 2, last_index + 3, last_index
                    ];

                    vertices.append(&mut new_vertices);
                    indices.append(&mut new_indices);
                    
                    (width,last_index + 4)
                },
                None => panic!("Character {} does not exist on CharacterSet",character_string)
            }
        });
        (vertices,indices)
    }

}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use crate::Binder;

    use super::{CharacterSet, Character};
    
    #[test]
    fn read_properly() {
        let set = CharacterSet::new("./assets/dzahui-font_test.fnt");
        let should_be_set = CharacterSet {
            characters: HashMap::from([
                ('{', Character::new(123,(0.0,0.0),(21.0,61.0),(1.0,13.0))),
                (' ',Character::new(32, (0.0,0.0), (0.0,0.0), (5.0,18.0))),
                ('a',Character::new(97, (211.0,153.0), (35.0,37.0), (2.0,25.0))),
            ]),
            font_type: "Liberation Sans".to_string(),
            font_size: 12,
            is_italic: false,
            is_bold: false,
            line_height: 19,
            encoding: "unicode".to_string(),
            texture_file: "dzahui-font.png".to_string(),
            texture_size: (640, 394),
            character_number: 3
        };
        assert!( set == should_be_set );
    }

    #[test]
    fn test_vertices_content() {
        let set = CharacterSet::new("./assets/dzahui-font_test.fnt");
        let (vertices, indices) = set.get_vertices_from_text("{a{{{a");
        // number os squares (quads) should be 6, equal to the number of chars in text
        assert!( indices.len()/6 == 6 );
        assert!( vertices.len()/20 == 6 );
    }
}