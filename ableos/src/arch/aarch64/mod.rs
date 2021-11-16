use core::ptr;

// mod panic;
pub mod drivers;
pub mod init;

pub mod serial;
use crate::arch::drivers::nrf52::{Level, Pins};
use core::ptr::write_volatile;
global_asm!(include_str!("boot.s"));

fn delay(ticks: usize) {
    static mut DUMMY: usize = 0;

    // Reduce the number of iterations when in debug mode
    #[cfg(debug_assertions)]
    let ticks = ticks / 128;

    for t in 0..ticks {
        // Prevent the optimizer from removing this loop
        unsafe {
            write_volatile(&mut DUMMY, t);
        }
    }
}
use crate::print;

#[no_mangle]
pub extern "C" fn not_main() {
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    for byte in b"ableOS Arm 64" {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }

    let gpios = Pins::take();
    let mut led = gpios.p0_31;

    loop {
        led.set_high();
        delay(2_000_000);

        led.set_low();
        delay(6_000_000);
    }

    led.set_push_pull_output(Level::Low);
    crate::kmain::kernel_main();

    sloop();
}

pub fn sloop() -> ! {
    loop {}
}
pub fn print() {
    for byte in b"ableOS Arm 64" {
        const UART0: *mut u8 = 0x0900_0000 as *mut u8;

        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
}

pub fn shutdown() {}
