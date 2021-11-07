use super::drivers::vga;
use super::interrupts;

#[macro_use]
use crate::kprintln;

pub fn init() {
   interrupts::init_idt();
   // init_alloc();
   use core::fmt::Write;
   kprintln!("Hello World{}", "!");

   // vga::print_something();
}
