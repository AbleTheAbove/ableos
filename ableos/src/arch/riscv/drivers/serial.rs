use core::fmt::Arguments;
use core::fmt::Error;

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {};
}
/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => {};
    ($fmt:expr) => {};
    ($fmt:expr, $($arg:tt)*) => {};
}

pub struct Serial123 {
    uart_data: u32,
}

impl Serial123 {
    pub fn out(&mut self, s: ::core::fmt::Arguments) {
        let uart_data = 0x10000000 as *mut u8;
        for c in b"Hello, world!\n" {
            unsafe { uart_data.write_volatile(*c) };
        }
    }
}

use spin::Mutex;

use lazy_static::lazy_static;
lazy_static! {
    pub static ref SERIAL: Mutex<Serial123> = {
        let serial_port = Serial123 {
            uart_data: 0x10000000,
        };
        Mutex::new(serial_port)
    };
}
