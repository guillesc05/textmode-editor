use raylib::ffi::Color;


pub fn color_from_tuple(tup: &(u8,u8,u8)) -> Color{
    Color::new(tup.0, tup.1, tup.2, 255)
}