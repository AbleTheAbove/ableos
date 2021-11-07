#![no_std]
#![no_main]
pub const ARCH: &'static str = "mipsr4000";
pub mod drivers;
static mut LIST: psp::Align16<[u32; 0x40000]> = psp::Align16([0; 0x40000]);
// Create a module named "sample_module" with version 1.0
psp::module!("ableos", 1, 0);

fn psp_main() {
    // println!("AbleOS booted on PSP");
    // todo

    // println!("{}", crate::experiments::systeminfo::format_system_info());
    // gl_basic();
    println!("{}", crate::time::kilotime::Kilosecond::from_sec(23944));
    let mut second = timer_update().seconds;

    loop {
        /*
        {
            let time = timer_update();
            // FIXME: this is a little broken
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
        */
    }
}

mod simple_graphics;
use simple_graphics::gl_basic;
mod timer;
use crate::arch::timer::timer_update;
use core::ffi::c_void;

use psp::{
    sys::{self, DisplayPixelFormat, GuState, TexturePixelFormat},
    vram_alloc::get_vram_allocator,
    {BUF_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH},
};
pub mod init;
use crate::println;
