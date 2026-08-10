use raylib::{drawing::{RaylibDraw, RaylibDrawHandle}, ffi::{Color, Rectangle, Vector2}, text::Font};

use crate::{textmode_info::CharInfo, utils::font_utils::CP_437_CHARS, window::editor::CANVAS_WIDTH_PROPORTION};

pub fn toolkit_logic(d: &mut RaylibDrawHandle, selected_character: &mut Option<CharInfo>, font: &Font){
    let screen_size = (d.get_screen_width(), d.get_screen_height());

    let toolkit_rect = Rectangle{
        x: screen_size.0 as f32 * CANVAS_WIDTH_PROPORTION,
        y: 0.0,
        width: (1.0 - CANVAS_WIDTH_PROPORTION) * screen_size.0 as f32,
        height: screen_size.1 as f32
    };

    let glyph_menu_width = toolkit_rect.width * 0.9;

    let mut glyph_menu_rect = Rectangle{
        width: glyph_menu_width,
        height: glyph_menu_width,
        x: toolkit_rect.x + toolkit_rect.width / 2.0 - glyph_menu_width / 2.0,
        y : toolkit_rect.y + toolkit_rect.height/ 2.0 - glyph_menu_width / 2.0
    };
    //Draw

    d.draw_rectangle(toolkit_rect.x as i32, toolkit_rect.y as i32, toolkit_rect.width as i32, toolkit_rect.height as i32, Color::new(100, 100, 100, 255));


    let symbol_width: f32 = glyph_menu_rect.width / 16.0;

    for (i, character) in CP_437_CHARS.chars().enumerate(){
        let char_rect = Rectangle{
            x: glyph_menu_rect.x + symbol_width * (i % 16) as f32,
            y: glyph_menu_rect.y + symbol_width* (i / 16) as f32,
            width: symbol_width,
            height: symbol_width
        };

        d.draw_text_pro(font, &character.to_string(), Vector2::new(char_rect.x, char_rect.y), Vector2::zero(), 0.0, symbol_width, 0.0, Color::WHITE);
    }

}