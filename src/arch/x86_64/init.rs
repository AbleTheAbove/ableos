use super::drivers::vga;
use super::{gdt, interrupts};

use crate::println;

pub fn init() {
   gdt::init();
   interrupts::init_idt();
   println!("Hello World{}", "!");
}
