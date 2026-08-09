use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct CharInfo{
    pub character: char,
    pub foreground_color: (u8, u8, u8),
    pub background_color: (u8, u8, u8)
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct TextmodeInfo{
    pub font: String,
    pub x_size: u32,
    pub y_size: u32,
    pub tile_info: Vec<Vec<Option<CharInfo>>>
}

impl TextmodeInfo{
    pub fn empty_info(x_size: u32, y_size: u32, font: String) -> Self{

        let mut tile_info = Vec::new();

        for j in 0..y_size{
            let mut new_vec: Vec<Option<CharInfo>> = Vec::new();
            new_vec.resize(x_size as usize, None);

            tile_info.push(new_vec);
        }
        
        TextmodeInfo { 
            font: font, 
            x_size: x_size, 
            y_size: y_size, 
            tile_info: tile_info}
    }
}