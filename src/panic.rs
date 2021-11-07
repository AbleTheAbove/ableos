use crate::kprintln;
use core::{intrinsics::abort, panic::PanicInfo};
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("{}", info);

    loop {}
}
