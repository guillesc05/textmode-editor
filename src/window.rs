use raylib::{ffi::Rectangle, prelude::*};
use raylib::{RaylibHandle, RaylibThread};

pub enum WelcomeScreenResponse{
    NewProject,
    LoadProject,
    Exit
}

fn centered_rectangle(draw_handle: &RaylibDrawHandle, size: Vector2) -> Rectangle{
    let half_screen_width = draw_handle.get_screen_width() as f32 / 2.0;
    let half_screen_height = draw_handle.get_screen_height() as f32 / 2.0;

    Rectangle { x: half_screen_width - size.x/2.0, y: half_screen_height - size.y / 2.0, width: size.x, height: size.y }
}

pub fn welcome_window(rl: &mut RaylibHandle, thread: &RaylibThread) -> WelcomeScreenResponse{
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.gui_set_style(GuiControl::DEFAULT, GuiDefaultProperty::TEXT_SIZE, 25);
        let res = d.gui_message_box(centered_rectangle(&d, Vector2::new(500.0, 500.0)), "Textmode Editor", "What do you want to do?", "NewProject;LoadProject");

        if res == 1{
            return WelcomeScreenResponse::NewProject
        }

        if res == 2{
            load_project();
        }

        d.clear_background(Color::BLACK);
    }

    WelcomeScreenResponse::Exit
}

pub fn new_canvas_popup(){

}

pub fn load_project(){
    let res = rfd::FileDialog::new().add_filter("Images", &["json"]).pick_file();

    match res{
        None => {

        },
        Some(path) =>{

        }
    }
    
}

pub fn editor_window(){

}