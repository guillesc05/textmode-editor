use std::fs;

use raylib::ffi::CSSPalette;
use raylib::{ffi::Rectangle, prelude::*};
use raylib::{RaylibHandle, RaylibThread};

use crate::rect_utils::{centered_rectangle, relative_rectangle_centered};
use crate::textmode_info::{self, TextmodeInfo};
use crate::window::StateChange::OpenCanvas;

#[derive(PartialEq)]
pub enum StateChange{
    WelcomeWindow,
    NewProject,
    OpenCanvas(TextmodeInfo),
    Exit
}

pub fn welcome_window(rl: &mut RaylibHandle, thread: &RaylibThread) -> StateChange{
    rl.gui_set_style(GuiControl::DEFAULT, GuiDefaultProperty::TEXT_SIZE, 25);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        let rectangle_box = centered_rectangle(&d, Vector2::new(d.get_screen_width() as f32 /2.0, d.get_screen_height() as f32 / 2.0));

        let res = d.gui_message_box(rectangle_box, "Textmode Editor", "What do you want to do?", "NewProject;LoadProject");

        if res == 1{
            return StateChange::NewProject
        }

        if res == 2{
            match load_project(){
                Some(textmode_info) =>{
                    return OpenCanvas(textmode_info)
                },
                None =>{
                    return StateChange::Exit
                }
            }
        }

        d.clear_background(Color::BLACK);
    }

    StateChange::Exit
}

pub fn new_canvas_popup(rl: &mut RaylibHandle, thread: &RaylibThread) -> StateChange{
    let mut x_value: i32 = 40;
    let mut y_value: i32 = 40;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        //Parent
        let parent_rect = centered_rectangle(&d, Vector2::new(d.get_screen_width() as f32/2.0, d.get_screen_height() as f32/2.0));
        d.gui_group_box(parent_rect, "New Project");

        //X input value
        let x_input_rect = relative_rectangle_centered(&parent_rect, Vector2::new(0.5, 0.25), Vector2::new(0.5, 0.15));

        let view_x = x_input_rect.check_collision_point_rec(d.get_mouse_position());
        d.gui_value_box(x_input_rect, "x: ", &mut x_value , 0, 1000, view_x);

        //Y input value
        let y_input_rect = relative_rectangle_centered(&parent_rect, Vector2::new(0.5, 0.5), Vector2::new(0.5, 0.15));

        let view_y = y_input_rect.check_collision_point_rec(d.get_mouse_position());
        d.gui_value_box(y_input_rect, "y: ", &mut y_value , 0, 1000, view_y);

        //Confirm button

        let confitm_button_rect = relative_rectangle_centered(&parent_rect, Vector2::new(0.5, 0.75), Vector2::new(0.5, 0.15));

        if d.gui_button(confitm_button_rect, "Confirm"){
            return OpenCanvas(TextmodeInfo::empty_info(x_value as u32, y_value as u32, "dungeon-mode.ttf".to_string()))
        }


        d.clear_background(Color::BLACK);
    }

    StateChange::Exit
}

pub fn load_project() -> Option<TextmodeInfo>{
    let res = rfd::FileDialog::new().add_filter("Images", &["json"]).pick_file();

    match res{
        None => {
            None
        },
        Some(path) =>{
            match fs::read_to_string(path.clone()){
            Err(err) =>{
                return None
            },
            Ok(file_string) =>{
                let textmode_info: Result<TextmodeInfo, serde_json::Error> = serde_json::from_str(&file_string);
                match textmode_info{
                    Err(err) =>{
                        return None
                    }
                    Ok(s) =>{ Some(s)}
                }
            }
        }
        }
    }
    
}

const INITIAL_PX_VAL: i32 = 50;
pub fn editor_window(textmode_info: TextmodeInfo, rl: &mut RaylibHandle, thread: &RaylibThread, delta: &f32) -> StateChange{

    let mut zoom_value = 1.0;
    let mut offset = Vector2::zero();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        let tile_width = zoom_value * INITIAL_PX_VAL as f32;

        let canvas_size = Vector2::new(textmode_info.x_size as f32 * tile_width as f32, textmode_info.y_size as f32 * tile_width as f32);

        let screen_offset = Vector2::new(d.get_screen_width() as f32 / 2.0, d.get_screen_height() as f32 / 2.0);

        for j in 0..textmode_info.y_size{
            for i in 0..textmode_info.x_size{

                let curr_tile = Rectangle{
                    x: i as f32 * INITIAL_PX_VAL as f32 * zoom_value + offset.x - canvas_size.x / 2.0 + screen_offset.x,
                    y : j as f32 * INITIAL_PX_VAL as f32 * zoom_value + offset.y -canvas_size.y / 2.0 + screen_offset.y,
                    width: tile_width,
                    height: tile_width
                };

                d.draw_rectangle_lines(curr_tile.x as i32, curr_tile.y as i32, curr_tile.width as i32, curr_tile.height as i32, Color::WHITE);
            }
        }


        d.clear_background(Color::BLACK);
    }

    StateChange::Exit
}