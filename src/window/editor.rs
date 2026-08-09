use raylib::{RaylibHandle, RaylibThread, drawing::RaylibDraw, ffi::{CSSPalette, Color, MouseButton, RaylibPalette, Rectangle, Vector2}};

use crate::{textmode_info::{self, CharInfo, TextmodeInfo}, window::StateChange};

const INITIAL_PX_VAL: i32 = 50;
const WHEEL_SPEED: f32 = 0.05;
pub fn editor_window(textmode_info: TextmodeInfo, rl: &mut RaylibHandle, thread: &RaylibThread) -> StateChange{

    let mut textmode_info = textmode_info;

    let mut zoom_value = 1.0;
    let mut offset = Vector2::zero();
    
    let font = rl.load_font(&thread, "dungeon-mode.ttf").unwrap();
    
    while !rl.window_should_close() {
        //Logic
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT){
            offset += rl.get_mouse_delta();
        }
        
        zoom_value += rl.get_mouse_wheel_move_v().y * WHEEL_SPEED;
        if zoom_value <= 0.0{
            zoom_value = 0.001;
        }
        
        let tile_width = zoom_value * INITIAL_PX_VAL as f32;
        
        let canvas_size = Vector2::new(textmode_info.x_size as f32 * tile_width as f32, textmode_info.y_size as f32 * tile_width as f32);
        
        let screen_offset = Vector2::new(rl.get_screen_width() as f32 / 2.0, rl.get_screen_height() as f32 / 2.0);

        let mut tile_hover: Option<(usize, usize)> = None;
        
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
                match &textmode_info.tile_info[j as usize][i as usize]{
                    Some(character_info) =>{
                        let foreground_color = character_info.foreground_color;
                        let background_color = character_info.background_color;

                        d.draw_rectangle(curr_tile.x as i32, 
                            curr_tile.y as i32, 
                            curr_tile.width as i32, 
                            curr_tile.height as i32, 
                            Color::new(background_color.0, background_color.1, background_color.2, 255));

                        d.draw_text_pro(&font, 
                            &character_info.character.to_string(), 
                            Vector2::new(curr_tile.x, curr_tile.y), 
                            Vector2::zero(), 
                            0.0, 
                            curr_tile.width, 
                            0.0, 
                            Color::new(foreground_color.0, foreground_color.1, foreground_color.2, 255));
                    },
                    None =>{

                    }
                }

                if curr_tile.check_collision_point_rec(d.get_mouse_position()){
                    tile_hover = Some((i as usize, j as usize));
                }

            }
        }

        if tile_hover != None && d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT){
            let pos = tile_hover.unwrap();

            textmode_info.tile_info[pos.1][pos.0] = Some(CharInfo{
                character: 'A',
                foreground_color: (255,255,255),
                background_color: (255,39,50)
            });
        }
    }

    StateChange::Exit
}