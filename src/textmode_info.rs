use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CharInfo{
    character: char,
    foreground_color: (u8, u8, u8),
    background_color: (u8, u8, u8)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TextmodeInfo{
    font: String,
    x_size: u32,
    y_size: u32,
    tile_info: Vec<Vec<Option<CharInfo>>>
}