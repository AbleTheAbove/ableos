use super::drivers::vga;
use super::interrupts;

use crate::println;

pub fn init() {
    interrupts::init_idt();
    println!("Hello World{}", "!");
}
