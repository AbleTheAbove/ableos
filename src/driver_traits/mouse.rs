pub enum MouseButton {
    LeftMB,
    RightMB,
}
pub struct VectorTwo {
    pub x: i32,
    pub y: i32,
}

pub trait Mouse {
    fn movement();
    fn button();
}
