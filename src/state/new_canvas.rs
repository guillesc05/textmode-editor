use raylib::{RaylibHandle, RaylibThread, drawing::RaylibDraw, ffi::{Color, Vector2}, rgui::{RaylibGuiContainers, RaylibGuiControls}};

use crate::{textmode_info::TextmodeInfo, utils::rect_utils::{centered_rectangle, relative_rectangle_centered}, state::StateChange};

pub fn new_canvas_window(rl: &mut RaylibHandle, thread: &RaylibThread) -> StateChange{
    let mut x_value: i32 = 40;
    let mut y_value: i32 = 40;
    
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

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
            return StateChange::OpenCanvas(TextmodeInfo::empty_info(
                x_value as u32, 
                y_value as u32, 
                "dungeon-mode.ttf".to_string()))
        }


    }

    StateChange::Exit
}