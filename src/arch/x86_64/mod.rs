pub const ARCH: &'static str = "x86_64";

use x86_64::instructions::hlt;
pub mod drivers;
pub mod init;
pub mod interrupts;
pub mod gdt;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    crate::kmain::kernel_main();
    sloop();

    loop {}
}

pub fn shutdown() {}
pub fn sloop() {
    loop {
        hlt();
    }
}
