use raylib::{ffi::Rectangle, prelude::*};

use crate::window::StateChange::{Exit, NewProject, OpenCanvas};

mod window;
mod textmode_info;

fn main() {

    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();

    let mut font = rl.load_font(&thread, "dungeon-mode.ttf").unwrap();

    let welcome_response = window::welcome_window(&mut rl, &thread);

    match welcome_response{
        NewProject =>{
            window::new_canvas_popup(&mut rl, &thread);
        },
        OpenCanvas(textmode_info) =>{

        }
        Exit =>{}
    }
}
