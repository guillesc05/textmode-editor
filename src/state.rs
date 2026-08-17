use raylib::{RaylibHandle, RaylibThread};

use crate::state::editor::editor_info::EditorInfo;
use crate::textmode_info::{TextmodeInfo};
use crate::state::editor::editor_window;
use crate::state::new_canvas::new_canvas_window;
use crate::state::welcome::welcome_window;

mod welcome;
mod new_canvas;
mod editor;

#[derive(PartialEq)]
pub enum StateChange{
    WelcomeWindow,
    NewProject,
    OpenCanvas(EditorInfo),
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
        StateChange::OpenCanvas(editor_info) =>{
            editor_window(editor_info.to_owned(), rl, &thread)
        }
        StateChange::Exit =>{StateChange::Exit}
    };

    *state = return_state;
}