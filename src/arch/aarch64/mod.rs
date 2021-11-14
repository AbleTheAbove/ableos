use core::ptr;

// mod panic;
pub mod drivers;
pub mod init;
pub mod serial;

global_asm!(include_str!("boot.s"));

#[no_mangle]
pub extern "C" fn not_main() {
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let out_str = b"AArch64 Bare Metal\n";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }

    sloop();
}

pub fn sloop() -> ! {
    loop {}
}
