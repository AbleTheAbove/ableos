#![allow(unused)]
pub enum GModes {
    Vga800x600,
    Custom(u16, u16),
}
pub type GCoord = usize;

// TODO remap to a bitmasked u32
/* REASON: More effecient memory wise so less overhead on the wasm memory
Current: u32+u32+u32
Proposed: u32 with bitmaps
*/

pub struct Rgb {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

impl Rgb {
    fn to_vga_color() {
        todo!();
    }
}
pub type RefreshRate = u8;

pub const REFRESH_RATE: u8 = 60;
pub type Resolution = (usize, usize);
pub type Point = (GCoord, GCoord);
pub struct FrameBuffer;
// [[Rgb; 5]; 5]
pub trait Graphics {
    fn put_line(coords_start: Point, coords_end: Point, thickness: u32, color: Rgb);
    fn put_rect(coords_start: Point, coords_end: Point, color: Rgb);
    fn put_circle(coords: Point, radius: u32);
    fn put_pixel(coords: Point, color: Rgb);
    fn put_triangle(coords_1: Point, coords_2: Point, coords_3: Point, thickness: u32, color: Rgb);
    fn paint_cursor(coords: Point);
    fn hide_cursor();
    fn show_cursor();
    /// Actually move the double buffer to the single buffer and "update" the screen
    fn draw();
    fn clear();
}
