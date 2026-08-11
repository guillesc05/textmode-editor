use raylib::{drawing::{RaylibDraw, RaylibDrawHandle}, ffi::{CSSPalette, Color, Rectangle, Vector2, rintf}, rgui::{RaylibGuiAdvanced, RaylibGuiControls}, text::Font};

use crate::{textmode_info::CharInfo, utils::{font_utils::CP_437_CHARS, rect_utils::{centered_rectangle, centered_rectangle_in_rect}}, window::editor::CANVAS_WIDTH_PROPORTION};

pub fn toolkit_logic(d: &mut RaylibDrawHandle, selected_character: &mut CharInfo, font: &Font){
    let screen_size = (d.get_screen_width(), d.get_screen_height());

    let toolkit_rect = Rectangle{
        x: screen_size.0 as f32 * CANVAS_WIDTH_PROPORTION,
        y: 0.0,
        width: (1.0 - CANVAS_WIDTH_PROPORTION) * screen_size.0 as f32,
        height: screen_size.1 as f32
    };

    d.draw_rectangle(toolkit_rect.x as i32, toolkit_rect.y as i32, toolkit_rect.width as i32, toolkit_rect.height as i32, Color::new(100, 100, 100, 255));

    let upper_half_rect = Rectangle{
        x: toolkit_rect.x,
        y: toolkit_rect.y,
        width:toolkit_rect.width,
        height: toolkit_rect.height/2.0
    };

    let mut lower_half_rect = upper_half_rect.clone();
    lower_half_rect.y += lower_half_rect.height;

    glyph_selector(d, &upper_half_rect, font, selected_character);

    let mut foreground_color = Color::new(selected_character.foreground_color.0, selected_character.foreground_color.1, selected_character.foreground_color.2, 255);
    let mut background_color = Color::new(selected_character.background_color.0, selected_character.background_color.1, selected_character.background_color.2, 255);

    let left_lower_rect = Rectangle{
        x: lower_half_rect.x,
        y: lower_half_rect.y,
        width: lower_half_rect.width / 2.0,
        height: lower_half_rect.height
    };
    selected_character.background_color = (background_color.r, background_color.g, background_color.b);
    

    let mut right_lower_rect = left_lower_rect.clone();
    right_lower_rect.x = right_lower_rect.x + right_lower_rect.width;

    color_selector(d, &right_lower_rect, "A", &mut selected_character.background_color);
}

fn glyph_selector(d: &mut RaylibDrawHandle, rect: &Rectangle, font: &Font, selected_character: &mut CharInfo){
    let glyph_menu_width = rect.width.min(rect.height) * 0.9;

    let mut glyph_menu_rect = centered_rectangle_in_rect(&rect, Vector2::new(glyph_menu_width, glyph_menu_width));

    let symbol_width: f32 = glyph_menu_rect.width / 16.0;

    let foreground_color = Color::new(selected_character.foreground_color.0, selected_character.foreground_color.1, selected_character.foreground_color.2, 255);
    let background_color = Color::new(selected_character.background_color.0, selected_character.background_color.1, selected_character.background_color.2, 255);

    for (i, character) in CP_437_CHARS.chars().enumerate(){
        let char_rect = Rectangle{
            x: glyph_menu_rect.x + symbol_width * (i % 16) as f32,
            y: glyph_menu_rect.y + symbol_width* (i / 16) as f32,
            width: symbol_width,
            height: symbol_width
        };

        d.draw_rectangle(char_rect.x as i32, char_rect.y as i32, char_rect.width as i32, char_rect.height as i32, background_color);

        d.draw_text_pro(font, &character.to_string(), Vector2::new(char_rect.x, char_rect.y), Vector2::zero(), 0.0, symbol_width, 0.0, foreground_color);

        if selected_character.character == character{
            let inverted_color= Color::new(255 - background_color.r, 255 - background_color.g, 255 - background_color.b, 255);
            d.draw_rectangle_lines_ex(char_rect, 3.0, inverted_color);
        }

        if d.is_mouse_button_pressed(raylib::ffi::MouseButton::MOUSE_BUTTON_LEFT) && 
            char_rect.check_collision_point_rec(d.get_mouse_position()) {
                selected_character.character = character;
            }
    }
}

fn color_selector(d: &mut RaylibDrawHandle, rect: &Rectangle, text: &str, color: &mut (u8,u8,u8)){
    
}