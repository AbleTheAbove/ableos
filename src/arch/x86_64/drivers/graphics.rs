use able_graphics_lib::{Graphics, Point, RGB};

pub struct x86_64GraphicsBuffer;
impl Graphics for x86_64GraphicsBuffer {
    fn put_line(coords_start: Point, coords_end: Point, thickness: u32, color: RGB) {
        todo!()
    }
    fn put_rect(coords_start: Point, coords_end: Point, color: RGB) {
        todo!()
    }
    fn put_circle(coords: Point, radius: u32) {
        todo!()
    }
    fn put_pixel(coords: Point, color: RGB) {
        todo!()
    }
    fn paint_cursor(coords: Point) {
        todo!()
    }
    fn hide_cursor() {
        todo!()
    }
    fn show_cursor() {
        todo!()
    }
    fn draw() {
        todo!()
    }
    fn clear() {
        todo!()
    }
}
