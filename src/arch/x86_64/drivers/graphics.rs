use crate::driver_traits::graphics::{Graphics, Point, RGB};
use cpuio::outw;

pub struct GraphicsBuffer;
impl Graphics for GraphicsBuffer {
    fn put_line(coords_start: Point, coords_end: Point, thickness: u32, color: RGB) {
        todo!()
    }
    fn put_rect(coords_start: Point, coords_end: Point, color: RGB) {
        todo!()
    }
    fn put_circle(coords: Point, radius: u32) {
        todo!()
    }
    fn put_triangle(coords_1: Point, coords_2: Point, coords_3: Point, thickness: u32, color: RGB) {
        todo!();
    }
    fn put_pixel(coords: Point, color: RGB) {
        todo!()
    }
    fn paint_cursor(coords: Point) {
        todo!()
    }
    fn hide_cursor() {
        unsafe {
            outw(0x0A, 0x3D4);
            outw(0x20, 0x3D5);
        }
    }
    fn show_cursor() {}
    fn draw() {}
    fn clear() {
        todo!()
    }
}
