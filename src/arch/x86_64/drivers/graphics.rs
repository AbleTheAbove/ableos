use able_graphics_lib::{Graphics, Point, RGB};
use cpuio::{inw, outb, outw};
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
        unsafe {
            outw(0x0A, 0x3D4);
            outw(0x20, 0x3D5);
        }
    }
    fn show_cursor() {}
    fn draw() {
        let vga_buffer = 0xb8000 as *mut u8;
        static HELLO: &[u8] = b"Running on x84_64";
        for (i, &byte) in HELLO.iter().enumerate() {
            unsafe {
                *vga_buffer.offset(i as isize * 2) = byte;
                *vga_buffer.offset(i as isize * 2 + 1) = 0xa;
            }
        }
    }
    fn clear() {
        todo!()
    }
}
