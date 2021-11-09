#![allow(clippy::print_literal)]
use super::drivers::vga;
use super::{gdt, interrupts};

use crate::println;

pub fn init() {
   gdt::init();
   interrupts::init_idt();
   unsafe { interrupts::PICS.lock().initialize() };
   x86_64::instructions::interrupts::enable();
   println!("Hello World{}", "!");
}
