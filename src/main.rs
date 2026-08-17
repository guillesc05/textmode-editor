use raylib::{RaylibHandle, RaylibThread, ffi::{Color, GuiControlProperty, GuiDefaultProperty}, rgui::RaylibGuiState};

use crate::state::{StateChange, state_func};

mod state;
mod textmode_info;

mod utils;

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