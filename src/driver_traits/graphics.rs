pub enum GModes {
    Vga800x600,
    Custom(u16, u16),
}

pub type GCoord = usize;
pub struct RGB {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

impl RGB {
    fn to_vga_color() {
        todo!();
    }
}

pub type RefreshRate = u8;
pub const REFRESH_RATE: u8 = 60;

pub type Resolution = (usize, usize);

pub type Point = (GCoord, GCoord);

pub struct FrameBuffer;
// [[RGB; 5]; 5]
pub trait Graphics {
    fn put_line(coords_start: Point, coords_end: Point, thickness: u32, color: RGB);
    fn put_rect(coords_start: Point, coords_end: Point, color: RGB);
    fn put_circle(coords: Point, radius: u32);
    fn put_pixel(coords: Point, color: RGB);
    fn put_triangle(coords_1: Point, coords_2: Point, coords_3: Point, thickness: u32, color: RGB);
    fn paint_cursor(coords: Point);
    fn hide_cursor();
    fn show_cursor();
    /// Actually move the double buffer to the single buffer and "update" the screen
    fn draw();
    fn clear();
}
