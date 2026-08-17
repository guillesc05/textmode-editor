use std::fs;

use raylib::{RaylibHandle, RaylibThread, drawing::RaylibDraw, ffi::{Color, GuiControl, GuiDefaultProperty, Vector2}, rgui::{RaylibGuiAdvanced, RaylibGuiState}};

use crate::{utils::rect_utils::centered_rectangle, textmode_info::TextmodeInfo, state::StateChange};

pub fn welcome_window(rl: &mut RaylibHandle, thread: &RaylibThread) -> StateChange{
    rl.gui_set_style(GuiControl::DEFAULT, GuiDefaultProperty::TEXT_SIZE, 25);
    rl.set_target_fps(144);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        let rectangle_box = centered_rectangle(&d, Vector2::new(d.get_screen_width() as f32 /2.0, d.get_screen_height() as f32 / 2.0));

        let res = d.gui_message_box(rectangle_box, "Textmode Editor", "What do you want to do?", "NewProject;LoadProject");

        if res == 1{
            return StateChange::NewProject
        }

        if res == 2{
            match load_project(){
                Some(textmode_info) =>{
                    return StateChange::OpenCanvas(textmode_info)
                },
                None =>{
                    return StateChange::Exit
                }
            }
        }

    }

    StateChange::Exit
}

fn load_project() -> Option<TextmodeInfo>{
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