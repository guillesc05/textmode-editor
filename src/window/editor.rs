use raylib::{RaylibHandle, RaylibThread, drawing::{RaylibDraw, RaylibDrawHandle}, ffi::{CSSPalette, Color, MouseButton, RaylibPalette, Rectangle, Vector2}, text::Font};

use crate::{textmode_info::{CharInfo, TextmodeInfo}, utils::font_utils::CP_437_CHARS, window::StateChange};

const INITIAL_PX_VAL: i32 = 50;
const WHEEL_SPEED: f32 = 0.05;
pub fn editor_window(textmode_info: TextmodeInfo, rl: &mut RaylibHandle, thread: &RaylibThread) -> StateChange{

    let mut textmode_info = textmode_info;

    let mut zoom_value = 1.0;
    let mut offset = Vector2::zero();
    
    let font = rl.load_font_ex(&thread, "dungeon-mode.ttf", 8, Some(CP_437_CHARS)).unwrap();
    
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
        d.clear_background(Color::GREY);

        for j in 0..textmode_info.y_size{
            for i in 0..textmode_info.x_size{

                //Tile position and scale in screen
                let curr_tile = Rectangle{
                    x: i as f32 * INITIAL_PX_VAL as f32 * zoom_value + offset.x - canvas_size.x / 2.0 + screen_offset.x,
                    y : j as f32 * INITIAL_PX_VAL as f32 * zoom_value + offset.y -canvas_size.y / 2.0 + screen_offset.y,
                    width: tile_width,
                    height: tile_width
                };

                d.draw_rectangle_lines(curr_tile.x as i32, curr_tile.y as i32, curr_tile.width as i32, curr_tile.height as i32, Color::WHITE);
                
                draw_tile(&textmode_info.tile_info[j as usize][i as usize], &font, &curr_tile, &mut d);

                if curr_tile.check_collision_point_rec(d.get_mouse_position()){
                    tile_hover = Some((i as usize, j as usize));
                }

            }
        }

        if tile_hover != None && d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT){
            let pos = tile_hover.unwrap();

            textmode_info.tile_info[pos.1][pos.0] = Some(CharInfo{
                character: '♥',
                foreground_color: (255,255,255),
                background_color: (255,39,50)
            });
        }
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