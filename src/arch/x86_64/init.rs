use super::{drivers::vga, interrupts};

use crate::println;
use core::fmt::Write;

pub fn init() {
    interrupts::init_idt();
    println!("Hello World{}", "!");
}
