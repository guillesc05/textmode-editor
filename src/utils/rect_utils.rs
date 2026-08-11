use raylib::{drawing::RaylibDrawHandle, ffi::{Rectangle, Vector2}};

pub fn centered_rectangle(draw_handle: &RaylibDrawHandle, size: Vector2) -> Rectangle{
    let half_screen_width = draw_handle.get_screen_width() as f32 / 2.0;
    let half_screen_height = draw_handle.get_screen_height() as f32 / 2.0;

    Rectangle { x: half_screen_width - size.x/2.0, y: half_screen_height - size.y / 2.0, width: size.x, height: size.y }
}

//Relative size and position goes from 0.0 to 1.0
pub fn relative_rectangle_centered(parent_rect: &Rectangle, relative_position: Vector2, relative_size: Vector2) -> Rectangle{
    let width = parent_rect.width * relative_size.x;
    let height = parent_rect.height * relative_size.y;

    Rectangle { x: parent_rect.x + parent_rect.width * relative_position.x - width / 2.0,
        y: parent_rect.y + parent_rect.height * relative_position.y - height / 2.0, 
        width: width, 
        height: height }

}

pub fn centered_rectangle_in_rect(parent_rect: &Rectangle, size: Vector2) -> Rectangle{
    let center_x= parent_rect.x + parent_rect.width/2.0;
    let center_y= parent_rect.y + parent_rect.height/2.0;
    Rectangle { 
        x: center_x- size.x / 2.0,
        y: center_y - size.y / 2.0, 
        width: size.x, 
        height: size.y }
}