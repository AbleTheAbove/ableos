pub enum PS2MouseButton {
    LeftMB,
    RightMB,
}

pub trait PS2Mouse {
    fn movement();
    fn button();
}
