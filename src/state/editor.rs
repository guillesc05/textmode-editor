use std::cmp::min;

use raylib::{RaylibHandle, RaylibThread, drawing::{RaylibDraw, RaylibDrawHandle}, ffi::{CSSPalette, Color, MouseButton, RaylibPalette, Rectangle, Vector2}, init, text::Font};

use crate::{textmode_info::{CharInfo, TextmodeInfo}, utils::{font_utils::CP_437_CHARS, rect_utils::{centered_rectangle, relative_rectangle_centered}}, state::{StateChange, editor::{canvas::CanvasInfo, editor_info::PaletteState}}};

mod canvas;
mod toolkit;
pub mod editor_info;

// The width that takes the editor canvas
const CANVAS_WIDTH_PROPORTION: f32 = 0.7;

pub fn editor_window(textmode_info: TextmodeInfo, rl: &mut RaylibHandle, thread: &RaylibThread) -> StateChange{
    let mut textmode_info = textmode_info;
    let font = rl.load_font_ex(&thread, "dungeon-mode.ttf", 8, Some(CP_437_CHARS)).unwrap();

    let initial_px_val = {
        let tile_width_x = rl.get_screen_width() as f32 / textmode_info.x_size as f32;
        let tile_width_y = rl.get_screen_height() as f32 / textmode_info.y_size as f32;

        min(tile_width_x as i32, tile_width_y as i32)
    };
    let mut canvas_info= CanvasInfo::new(canvas::get_canvas_rect(rl.get_screen_height(), rl.get_screen_height()), initial_px_val);

    let mut selected_character = CharInfo { character: 'A', foreground_color: (255,255,255), background_color: (0,0,0) };

    let mut color_palette = PaletteState::new();
    
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::GREY);
        d.draw_fps(0, 0);

        canvas::canvas_logic(&mut canvas_info, &mut textmode_info, &selected_character, &font, &mut d, thread);

        toolkit::toolkit_logic(&mut d, &mut selected_character, &font, &mut color_palette);
    }

    StateChange::Exit
}

fn draw_tile(character_info: &Option<CharInfo>, font: &Font,rect: &Rectangle, d: &mut RaylibDrawHandle){
    match character_info{
                    Some(res) =>{
                        let foreground_color = res.foreground_color;
                        let background_color = res.background_color;

                        d.draw_rectangle(rect.x as i32, 
                            rect.y as i32, 
                            rect.width as i32, 
                            rect.height as i32, 
                            Color::new(background_color.0, background_color.1, background_color.2, 255));

                        d.draw_text_pro(&font, 
                            &res.character.to_string(), 
                            Vector2::new(rect.x, rect.y), 
                            Vector2::zero(), 
                            0.0, 
                            rect.width, 
                            0.0, 
                            Color::new(foreground_color.0, foreground_color.1, foreground_color.2, 255));
                    },
                    None =>{
                        let width = (rect.width / 2.0) as i32;
                        let height = (rect.height / 2.0) as i32;
                        d.draw_rectangle(rect.x as i32, rect.y as i32, width, height, Color::GRAY);
                        d.draw_rectangle(rect.x as i32 + width, rect.y as i32, width, height, Color::WHITE);
                        d.draw_rectangle(rect.x as i32, rect.y as i32 + height, width, height, Color::WHITE);
                        d.draw_rectangle(rect.x as i32 + width, rect.y as i32 + height, width, height, Color::GRAY);
                    }
                }
}