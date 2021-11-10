pub type Priority = [u8; 0];

pub struct Scheduler {
    pub high_priority: Priority,   //150
    pub medium_priority: Priority, //100
    pub low_priority: Priority,    // 50
}

impl Scheduler {
    pub fn bump_up() {}
    pub fn bump_down() {}
    pub fn schedule() {}
}
