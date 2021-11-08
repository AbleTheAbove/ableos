//! used to give a base line example of windows and window handling
use crate::driver_traits::graphics::Point;
pub struct Window {
    // title: String,
    position: Point,
    fullscreen: bool,
}
// all of these should return a result
impl Window {
    pub fn fullscreen(&mut self) -> Result<(), u8> {
        self.fullscreen = true;
        Ok(())
    }
    pub fn revert_fullscreen(&mut self) {
        self.fullscreen = false;
    }
    pub fn set_title(&mut self) {
        todo!();
    }
    pub fn set_position(&mut self, pos: Point) {
        self.position = pos;
    }
}
