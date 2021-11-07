use super::drivers::vga;

use crate::println;

pub fn init() {
    // init_alloc();
    use core::fmt::Write;
    println!("Hello World{}", "!");
}
