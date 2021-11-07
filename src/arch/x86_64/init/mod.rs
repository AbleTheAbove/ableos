use super::drivers::vga;
pub mod interrupts;

#[macro_use]
use crate::kprintln;

pub fn init() {

   // init_alloc();
   use core::fmt::Write;
   kprintln!("Hello World{}", "!");

   // vga::print_something();
}
