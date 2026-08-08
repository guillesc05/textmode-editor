use raylib::{RaylibHandle, RaylibThread};

use crate::window::{StateChange, state_func};

mod window;
mod textmode_info;
mod rect_utils;

fn main() {

    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();
    

    let mut current_state = StateChange::WelcomeWindow;

    while current_state != StateChange::Exit{
        state_func(&mut current_state, &mut rl, &thread);
    }
}