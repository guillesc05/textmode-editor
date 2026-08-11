use raylib::{RaylibHandle, RaylibThread, drawing::{RaylibDraw, RaylibDrawHandle}, ffi::{CSSPalette, Color, MouseButton, Rectangle, Vector2}, text::Font};

use crate::{textmode_info::{CharInfo, TextmodeInfo}, window::editor::{CANVAS_WIDTH_PROPORTION, draw_tile}};

const WHEEL_SPEED: f32 = 0.05;

pub struct CanvasInfo{  
    pub zoom_value: f32,
    pub offset: Vector2,
    pub canvas_rect: Rectangle,
    pub initial_px_value: i32
}

impl CanvasInfo{
    pub fn new(canvas_rect: Rectangle, initial_px_value: i32) -> Self{
        CanvasInfo { 
            zoom_value: 1.0, 
            offset: Vector2::zero(), 
            canvas_rect: canvas_rect.clone(),
            initial_px_value: initial_px_value
        }
    }
}

pub fn get_canvas_rect(screen_width: i32, screen_height: i32) -> Rectangle{
        Rectangle { 
            x:0.0, 
            y: 0.0, 
            width: screen_width as f32 * CANVAS_WIDTH_PROPORTION, 
            height: screen_height as f32 }
    }
pub fn canvas_logic(canvas_info: &mut CanvasInfo, textmode_info: &mut TextmodeInfo, selected_character: &CharInfo, font: &Font, d: &mut RaylibDrawHandle, thread: &RaylibThread){
    //Logic
        if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT){
            canvas_info.offset += d.get_mouse_delta();
        }
        
        canvas_info.zoom_value += d.get_mouse_wheel_move_v().y * WHEEL_SPEED;
        if canvas_info.zoom_value <= 0.0{
            canvas_info.zoom_value = 0.001;
        }
        
        let tile_width = canvas_info.zoom_value * canvas_info.initial_px_value as f32;
        
        let canvas_size = Vector2::new(textmode_info.x_size as f32 * tile_width as f32, textmode_info.y_size as f32 * tile_width as f32);
        
        let screen_offset = Vector2::new(d.get_screen_width() as f32 / 2.0, d.get_screen_height() as f32 / 2.0);

        let mut tile_hover: Option<(usize, usize)> = None;
        
        //Draw
        for j in 0..textmode_info.y_size{
            for i in 0..textmode_info.x_size{

                //Tile position and scale in screen
                let curr_tile = Rectangle{
                    x: i as f32 * canvas_info.initial_px_value as f32 * canvas_info.zoom_value + canvas_info.offset.x - canvas_size.x / 2.0 + screen_offset.x,
                    y : j as f32 * canvas_info.initial_px_value as f32 * canvas_info.zoom_value + canvas_info.offset.y -canvas_size.y / 2.0 + screen_offset.y,
                    width: tile_width,
                    height: tile_width
                };

                d.draw_rectangle_lines(curr_tile.x as i32, curr_tile.y as i32, curr_tile.width as i32, curr_tile.height as i32, Color::WHITE);
                
                draw_tile(&textmode_info.tile_info[j as usize][i as usize], &font, &curr_tile, d);

                if curr_tile.check_collision_point_rec(d.get_mouse_position()) && 
                    get_canvas_rect(d.get_screen_width(), d.get_screen_height()).check_collision_point_rec(d.get_mouse_position()){
                    tile_hover = Some((i as usize, j as usize));
                }

            }
        }

        if tile_hover != None && d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT){
            let pos = tile_hover.unwrap();

            textmode_info.tile_info[pos.1][pos.0] = Some(selected_character.clone());
        }
}