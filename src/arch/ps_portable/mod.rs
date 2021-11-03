#![no_std]
#![no_main]

// Create a module named "sample_module" with version 1.0
psp::module!("ableos", 1, 0);
fn psp_main() {
    println!("AbleOS booted on PSP");
}

pub mod init;
use psp::{dprint as print, dprintln as println};
