#![allow(warnings)]
pub const ARCH: &'static str = "aarch32";
use core::intrinsics::volatile_load;
use core::intrinsics::volatile_store;

pub mod drivers;
pub mod init;
pub mod mbox;

// raspi2 and raspi3 have peripheral base address 0x3F000000,
// b ut raspi1 has peripheral base address 0x20000000. Ensure
// you are using the correct peripheral address for your
// hardware.
const UART_DR: u32 = 0x3F201000;
const UART_FR: u32 = 0x3F201018;

pub fn mmio_write(reg: u32, val: u32) {
    unsafe { volatile_store(reg as *mut u32, val) }
}

pub fn mmio_read(reg: u32) -> u32 {
    unsafe { volatile_load(reg as *const u32) }
}
pub fn transmit_fifo_full() -> bool {
    mmio_read(UART_FR) & (1 << 5) > 0
}
pub fn receive_fifo_empty() -> bool {
    mmio_read(UART_FR) & (1 << 4) > 0
}
pub fn writec(c: u8) {
    while transmit_fifo_full() {}
    mmio_write(UART_DR, c as u32);
}
pub fn getc() -> u8 {
    while receive_fifo_empty() {}
    mmio_read(UART_DR) as u8
}
pub fn write(msg: &str) {
    for c in msg.chars() {
        writec(c as u8)
    }
}

pub fn shutdown() {}
