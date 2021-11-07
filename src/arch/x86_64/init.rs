use super::drivers::vga;
use super::interrupts;

use crate::println;

pub fn init() {
    interrupts::init_idt();
    // init_alloc();
    use core::fmt::Write;
    println!("Hello World{}", "!");

    // vga::print_something();
}
