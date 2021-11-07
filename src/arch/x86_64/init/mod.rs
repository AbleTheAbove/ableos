use super::drivers::vga;
pub mod interrupts;

use crate::println;

pub fn init() {
    // init_alloc();
    use core::fmt::Write;
    println!("Hello World{}", "!");
}
