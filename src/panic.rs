use core::{intrinsics::abort, panic::PanicInfo};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    abort()
}
