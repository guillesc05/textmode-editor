use raylib::{RaylibHandle, RaylibThread};

use crate::window::{StateChange::{self, Exit, NewProject, OpenCanvas, WelcomeWindow}, editor_window};

mod window;
mod textmode_info;
mod rect_utils;

fn main() {

    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();
    

    let mut current_state = WelcomeWindow;

    while current_state != Exit{
        state_func(&mut current_state, &mut rl, &thread);
    }
}

fn state_func(state: &mut StateChange, rl: &mut RaylibHandle, thread: &RaylibThread ){
    let return_state = match state{
        WelcomeWindow => {
            window::welcome_window(rl, thread)
        },
        NewProject =>{
            window::new_canvas_popup(rl, &thread)
        },
        OpenCanvas(textmode_info) =>{
            editor_window(textmode_info.to_owned(), rl, &thread, &rl.get_frame_time())
        }
        Exit =>{Exit}
    };

    *state = return_state;
}