use raylib::{RaylibHandle, RaylibThread};

use crate::textmode_info::{TextmodeInfo};
use crate::window::editor::editor_window;
use crate::window::new_canvas::new_canvas_window;
use crate::window::welcome::welcome_window;

mod welcome;
mod new_canvas;
mod editor;

#[derive(PartialEq)]
pub enum StateChange{
    WelcomeWindow,
    NewProject,
    OpenCanvas(TextmodeInfo),
    Exit
}

pub fn state_func(state: &mut StateChange, rl: &mut RaylibHandle, thread: &RaylibThread ){
    let return_state = match state{
        StateChange::WelcomeWindow => {
            welcome_window(rl, thread)
        },
        StateChange::NewProject =>{
            new_canvas_window(rl, &thread)
        },
        StateChange::OpenCanvas(textmode_info) =>{
            editor_window(textmode_info.to_owned(), rl, &thread, &rl.get_frame_time())
        }
        StateChange::Exit =>{StateChange::Exit}
    };

    *state = return_state;
}