mod alloc;
pub mod drivers;
pub mod init;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    crate::kmain::kernel_main();
    loop {}
}

pub fn shutdown() {}
