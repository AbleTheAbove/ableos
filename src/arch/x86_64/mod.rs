pub const ARCH: &'static str = "x86_64";
pub mod drivers;
pub mod init;
pub mod interrupts;
pub mod gdt;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    crate::kmain::kernel_main();
    loop {}
}

pub fn shutdown() {}
