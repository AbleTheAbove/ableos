//! used to give a base line example of windows and window handling
use crate::driver_traits::graphics::Point;
pub struct Window {
    // title: String,
    position: Point,
    fullscreen: bool,
}

impl Window {
    pub fn fullscreen(&mut self) {
        self.fullscreen = true;
    }
    pub fn revert_fullscreen(&mut self) {
        self.fullscreen = false;
    }
    pub fn set_title(&mut self) {}
}
