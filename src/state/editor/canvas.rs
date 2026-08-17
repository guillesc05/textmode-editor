use raylib::{RaylibHandle, RaylibThread, drawing::{RaylibDraw, RaylibDrawHandle}, ffi::{CSSPalette, Color, MouseButton, Rectangle, Vector2}, text::Font};

use crate::{state::editor::{CANVAS_WIDTH_PROPORTION, draw_tile, editor_info::{CanvasInfo, EditorInfo}}, textmode_info::{CharInfo, TextmodeInfo}};

const WHEEL_SPEED: f32 = 0.05;

fn get_canvas_rect(screen_width: i32, screen_height: i32) -> Rectangle{
        Rectangle { 
            x:0.0, 
            y: 0.0, 
            width: screen_width as f32 * CANVAS_WIDTH_PROPORTION, 
            height: screen_height as f32 }
    }


pub fn canvas_logic(editor_info: &mut EditorInfo, font: &Font, d: &mut RaylibDrawHandle){
    //Logic
        if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT){
            let delta = d.get_mouse_delta();
            let curr_offset = editor_info.canvas_info.offset;
            editor_info.canvas_info.offset = (curr_offset.0 + delta.x, curr_offset.1 + delta.y);
        }
        
        editor_info.canvas_info.zoom_value += d.get_mouse_wheel_move_v().y * WHEEL_SPEED;
        if editor_info.canvas_info.zoom_value <= 0.0{
            editor_info.canvas_info.zoom_value = 0.001;
        }
        
        let tile_width = editor_info.canvas_info.zoom_value * editor_info.canvas_info.initial_px_value as f32;
        
        let canvas_size = Vector2::new(editor_info.textmode_info.x_size as f32 * tile_width as f32, editor_info.textmode_info.y_size as f32 * tile_width as f32);
        
        let screen_offset = Vector2::new(d.get_screen_width() as f32 / 2.0, d.get_screen_height() as f32 / 2.0);

        let mut tile_hover: Option<(usize, usize)> = None;

        let canvas_rect = get_canvas_rect(d.get_screen_width(), d.get_screen_height());
        
        //Draw
        for j in 0..editor_info.textmode_info.y_size{
            for i in 0..editor_info.textmode_info.x_size{

                //Tile position and scale in screen
                let curr_tile = Rectangle{
                    x: i as f32 * editor_info.canvas_info.initial_px_value as f32 * editor_info.canvas_info.zoom_value + editor_info.canvas_info.offset.0 - canvas_size.x / 2.0 + screen_offset.x,
                    y : j as f32 * editor_info.canvas_info.initial_px_value as f32 * editor_info.canvas_info.zoom_value + editor_info.canvas_info.offset.1 -canvas_size.y / 2.0 + screen_offset.y,
                    width: tile_width,
                    height: tile_width
                };

                d.draw_rectangle_lines(curr_tile.x as i32, curr_tile.y as i32, curr_tile.width as i32, curr_tile.height as i32, Color::WHITE);
                
                draw_tile(&editor_info.textmode_info.tile_info[j as usize][i as usize], &font, &curr_tile, d);

                if curr_tile.check_collision_point_rec(d.get_mouse_position()) && 
                    canvas_rect.check_collision_point_rec(d.get_mouse_position()){
                    tile_hover = Some((i as usize, j as usize));
                }

            }
        }

        if tile_hover != None && d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT){
            let pos = tile_hover.unwrap();

            editor_info.textmode_info.tile_info[pos.1][pos.0] = Some(editor_info.selected_character.clone());
        }
}