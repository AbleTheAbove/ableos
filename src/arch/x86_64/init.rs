use super::alloc::init_alloc;
use super::drivers::vga;

#[macro_use]
use crate::kprintln;

pub fn init() {
    // init_alloc();
    use core::fmt::Write;
    kprintln!("Hello World{}", "!");

    // vga::print_something();
}
