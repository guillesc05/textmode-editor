use std::fs;

use raylib::{ffi::Rectangle, prelude::*};
use raylib::{RaylibHandle, RaylibThread};

use crate::textmode_info::{self, TextmodeInfo};
use crate::window::StateChange::OpenCanvas;

pub enum StateChange{
    NewProject,
    OpenCanvas(TextmodeInfo),
    Exit
}

fn centered_rectangle(draw_handle: &RaylibDrawHandle, size: Vector2) -> Rectangle{
    let half_screen_width = draw_handle.get_screen_width() as f32 / 2.0;
    let half_screen_height = draw_handle.get_screen_height() as f32 / 2.0;

    Rectangle { x: half_screen_width - size.x/2.0, y: half_screen_height - size.y / 2.0, width: size.x, height: size.y }
}

pub fn welcome_window(rl: &mut RaylibHandle, thread: &RaylibThread) -> StateChange{
    rl.gui_set_style(GuiControl::DEFAULT, GuiDefaultProperty::TEXT_SIZE, 25);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        let res = d.gui_message_box(centered_rectangle(&d, Vector2::new(500.0, 500.0)), "Textmode Editor", "What do you want to do?", "NewProject;LoadProject");

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

pub fn new_canvas_popup(rl: &mut RaylibHandle, thread: &RaylibThread){
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.gui_message_box(centered_rectangle(&d, Vector2::new(400.0, 200.0)), "New project", "Insert sizes of project", "");

        d.clear_background(Color::BLACK);

    }
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

pub fn editor_window(){

}