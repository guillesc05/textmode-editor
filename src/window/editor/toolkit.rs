use raylib::{drawing::{RaylibDraw, RaylibDrawHandle}, ffi::{CSSPalette, Color, RaylibPalette, Rectangle, Vector2, rintf}, rgui::{RaylibGuiAdvanced, RaylibGuiControls}, text::Font};

use crate::{textmode_info::CharInfo, utils::{font_utils::CP_437_CHARS, rect_utils::{centered_rectangle, centered_rectangle_in_rect, relative_rectangle_centered}}, window::editor::CANVAS_WIDTH_PROPORTION};

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
    color_selector(d, &lower_half_rect, selected_character);
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

fn color_selector(d: &mut RaylibDrawHandle, rect: &Rectangle, selected_color: &mut CharInfo){
    let selected_color_rect_width = (rect.height * 0.25).min(rect.width * 0.25);
    let selected_color_rect = Rectangle{
        height: selected_color_rect_width,
        width: selected_color_rect_width,
        x: rect.x + (rect.width / 2.0) - selected_color_rect_width / 2.0,
        ..*rect
    };

    let color_palette_rect = Rectangle{
        y: rect.y + selected_color_rect.height,
        height: rect.height - selected_color_rect.height,
        ..*rect
    };

    let foreground_color = Color::new(
        selected_color.foreground_color.0, selected_color.foreground_color.1, selected_color.foreground_color.2, 255);

    let background_color = Color::new(
        selected_color.background_color.0, selected_color.background_color.1, selected_color.background_color.2, 255);

    d.draw_rectangle((selected_color_rect.x + selected_color_rect.width * 0.34) as i32, (selected_color_rect.y + selected_color_rect.height * 0.34) as i32, (selected_color_rect.width * 0.66) as i32, (selected_color_rect.height * 0.66) as i32, background_color);
    d.draw_rectangle(selected_color_rect.x as i32, selected_color_rect.y as i32, (selected_color_rect.width * 0.66) as i32, (selected_color_rect.height * 0.66) as i32, foreground_color);

    let color_swap_rect = relative_rectangle_centered(&Rectangle{
        x: selected_color_rect.x + selected_color_rect.width,
        ..selected_color_rect}, Vector2::new(0.5, 0.5), Vector2::new(0.5, 0.5));

    let color_swap = d.gui_button(color_swap_rect
    , "s");


    if color_swap {
        let aux = selected_color.background_color;
        selected_color.background_color = selected_color.foreground_color;
        selected_color.foreground_color = aux;
    }
    

}