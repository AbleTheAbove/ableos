#![no_std]
#![no_main]

static mut LIST: psp::Align16<[u32; 0x40000]> = psp::Align16([0; 0x40000]);
// Create a module named "sample_module" with version 1.0
psp::module!("ableos", 1, 0);

fn psp_main() {
    println!("AbleOS booted on PSP");

    gl_basic();
    let mut second = timer_update().seconds;

    loop {
        {
            let time = timer_update();
            if (second) == time.seconds {
                if second == 59 {
                    second = 0;
                }
                // time
                print!(
                    "{:?}/{:?}/{:?} {:02}:{:02}:{:02} UTC\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n",
                    time.year, time.month, time.day, time.hour, time.minutes, time.seconds
                );
                second += 1;
            }
        }
    }
}

mod simple_graphics;
use simple_graphics::gl_basic;
mod timer;
use crate::arch::timer::timer_update;
use core::ffi::c_void;

use psp::{
    dprint as print, dprintln as println,
    sys::{self, DisplayPixelFormat, GuState, TexturePixelFormat},
    vram_alloc::get_vram_allocator,
    {BUF_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH},
};
pub mod init;
