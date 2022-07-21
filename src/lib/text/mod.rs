use std::{fs::File,collections::HashMap,io::{BufReader, BufRead}};

#[derive(Debug)]
struct Character {
    // Character id
    id: usize,
    // Where it starts (top left corner)
    origin: (f32,f32),
    // Width and height of texture representing character
    size: (f32,f32),
    // Offset from top left corner
    character_start: (f32,f32),
    // Height and width of character
    character_size: (f32,f32),
}

impl Character {
    pub fn new(id: usize, origin: (f32,f32), size: (f32,f32), character_start: (f32,f32), character_size: (f32,f32)) -> Self {
        Self { id, origin, size, character_start, character_size }
    }
}

#[derive(Debug)]
#[derive(PartialEq, Eq)]
struct CharacterSet {
    characters: HashMap<String, Character>,
    font_type: String,
    font_size: u32, // pt
    is_italic: bool,
    is_bold: bool,
    line_height: u32, // Pixels
    encoding: String,
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

        // Processing rest of file to create the characters
        for line in reader {
            match line {
                Ok(content) => {
                    
                },
                Err(e) => panic!("Unable to read file propperly: {}",e)
            }
        }
        
        Self { 
            font_type, 
            font_size: property_map_one.remove("size").expect("Size parameter not found").parse().unwrap(),
            is_bold: property_map_one.get("bold").expect("Bold parameter not found") == "1",
            is_italic: property_map_one.get("bold").expect("Bold parameter not found") == "1",
            encoding: String::from("unicode"),
            line_height: property_map_two.remove("lineHeight").expect("Line height property not found").parse().unwrap(),
            texture_size: (
                property_map_two.remove("scaleW").expect("Line height property not found").parse().unwrap(),
                property_map_two.remove("scaleH").expect("Line height property not found").parse().unwrap(),
            ),
            texture_file: property_map_three.remove("file").expect("File property not found.").replace("\"","")
         }
    }

}

#[cfg(test)]
mod test {
    use super::CharacterSet;
    #[test]
    fn read_properly() {
        let set = CharacterSet::new("/home/Arthur/Tesis/Dzahui/assets/dzahui-font_0.fnt");
        let should_be_set = CharacterSet {
            font_type: "Liberation Sans".to_string(),
            font_size: 12,
            is_italic: false,
            is_bold: false,
            line_height: 19,
            encoding: "unicode".to_string(),
            texture_file: "dzahui-font.png".to_string(),
            texture_size: (640, 394)
        };
        assert!( set == should_be_set );
    }
}