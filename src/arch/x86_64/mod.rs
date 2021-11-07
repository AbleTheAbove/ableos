pub const ARCH: &'static str = "x86_64";
mod alloc;
pub mod drivers;
pub mod init;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    crate::kmain::kernel_main();
    loop {}
}
