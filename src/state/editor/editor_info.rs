use raylib::{drawing::{RaylibDraw, RaylibDrawHandle}, ffi::{Color, Rectangle}};

use crate::{textmode_info::TextmodeInfo, utils::color_utils::color_from_tuple};

//Data from the editor
struct EditorInfo{
    textmode_info: TextmodeInfo,
    palette_state: PaletteState
}

pub struct PaletteState{
    pub colours_saved: [(u8,u8,u8); Self::MAX_COLORS as usize]
}

impl PaletteState{

    pub fn new() -> Self{
        PaletteState { colours_saved: [(255,255,255); Self::MAX_COLORS as usize]}
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