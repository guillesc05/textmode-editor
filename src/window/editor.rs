use raylib::{RaylibHandle, RaylibThread, drawing::RaylibDraw, ffi::{CSSPalette, Color, MouseButton, Rectangle, Vector2}};

use crate::{textmode_info::TextmodeInfo, window::StateChange};

const INITIAL_PX_VAL: i32 = 50;
const WHEEL_SPEED: f32 = 0.2;
pub fn editor_window(textmode_info: TextmodeInfo, rl: &mut RaylibHandle, thread: &RaylibThread) -> StateChange{

    let mut zoom_value = 1.0;
    let mut offset = Vector2::zero();
    
    let font = rl.load_font(&thread, "dungeon-mode.ttf").unwrap();
    
    while !rl.window_should_close() {
        //Logic
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT){
            offset += rl.get_mouse_delta();
        }
        
        zoom_value += rl.get_mouse_wheel_move_v().y * WHEEL_SPEED;
        if zoom_value <= 0.0{
            zoom_value = 0.001;
        }
        
        //zoom_value += delta * 0.5;
        
        let tile_width = zoom_value * INITIAL_PX_VAL as f32;
        
        let canvas_size = Vector2::new(textmode_info.x_size as f32 * tile_width as f32, textmode_info.y_size as f32 * tile_width as f32);
        
        let screen_offset = Vector2::new(rl.get_screen_width() as f32 / 2.0, rl.get_screen_height() as f32 / 2.0);
        
        //Draw
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        for j in 0..textmode_info.y_size{
            for i in 0..textmode_info.x_size{

                let curr_tile = Rectangle{
                    x: i as f32 * INITIAL_PX_VAL as f32 * zoom_value + offset.x - canvas_size.x / 2.0 + screen_offset.x,
                    y : j as f32 * INITIAL_PX_VAL as f32 * zoom_value + offset.y -canvas_size.y / 2.0 + screen_offset.y,
                    width: tile_width,
                    height: tile_width
                };

                d.draw_rectangle_lines(curr_tile.x as i32, curr_tile.y as i32, curr_tile.width as i32, curr_tile.height as i32, Color::WHITE);

                d.draw_text_pro(&font, "A", Vector2::new(curr_tile.x, curr_tile.y), Vector2::zero(), 0.0, curr_tile.width, 0.0, Color::WHITE);
            }
        }
    }

    StateChange::Exit
}