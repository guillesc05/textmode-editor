use std::cmp::min;

use raylib::{RaylibHandle, drawing::{RaylibDraw, RaylibDrawHandle}, ffi::{Color, Rectangle, Vector2}};
use serde::{Deserialize, Serialize};

use crate::{textmode_info::{CharInfo, TextmodeInfo}, utils::color_utils::color_from_tuple};

//Data from the editor
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct EditorInfo{
    pub textmode_info: TextmodeInfo,
    pub palette_state: PaletteInfo,
    pub selected_character: CharInfo,
    pub canvas_info: CanvasInfo
}

impl EditorInfo{
    pub fn new(width: u32, height: u32, font: String, rl: &RaylibHandle) -> Self{
        let textmode_info = TextmodeInfo::empty_info(width, height, font);
        let initial_px_val = {
            let tile_width_x = rl.get_screen_width() as f32 / textmode_info.x_size as f32;
            let tile_width_y = rl.get_screen_height() as f32 / textmode_info.y_size as f32;

            min(tile_width_x as i32, tile_width_y as i32)
        };
        let canvas_info= CanvasInfo::new(initial_px_val);
        let selected_character = CharInfo { character: 'A', foreground_color: (255,255,255), background_color: (0,0,0) };
        let color_palette = PaletteInfo::new();

        EditorInfo { 
            textmode_info: textmode_info, 
            palette_state: color_palette, 
            selected_character: selected_character, 
            canvas_info: canvas_info 
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct PaletteInfo{
    pub colours_saved: [(u8,u8,u8); Self::MAX_COLORS as usize]
}

impl PaletteInfo{

    pub fn new() -> Self{
        PaletteInfo { colours_saved: [(255,255,255); Self::MAX_COLORS as usize]}
    }

    const COLORS_PER_ROW: i32 = 4;
    const MAX_COLORS: i32 = 32;
    const COLORS_PER_COL: i32 = Self::MAX_COLORS / Self::COLORS_PER_ROW;

    pub fn render(&self, d: &mut RaylibDrawHandle, rect: &Rectangle){

        let rect_width = rect.width / Self::COLORS_PER_ROW as f32;
        let rect_height = rect.height / Self::COLORS_PER_COL as f32;

        for (index,color) in self.colours_saved.iter().enumerate(){
            let i = index as i32 % Self::COLORS_PER_ROW;
            let j = index as i32 / Self::COLORS_PER_ROW;

            let col_rect = Rectangle{
                x: rect.x + i as f32 * rect_width,
                y: rect.y + j as f32 * rect_height,
                width: rect_width,
                height: rect_width
            };

            let color_r = color_from_tuple(color);

            d.draw_rectangle(col_rect.x as i32, col_rect.y as i32, col_rect.width as i32, col_rect.height as i32, color_r);
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct CanvasInfo{  
    pub zoom_value: f32,
    pub offset: (f32,f32),
    pub initial_px_value: i32
}

impl CanvasInfo{
    pub fn new(initial_px_value: i32) -> Self{
        CanvasInfo { 
            zoom_value: 1.0, 
            offset: (0.0,0.0), 
            initial_px_value: initial_px_value
        }
    }
}