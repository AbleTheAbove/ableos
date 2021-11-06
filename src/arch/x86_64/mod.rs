mod alloc;
pub mod drivers;
pub mod init;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
