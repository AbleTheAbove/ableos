use crate::println;
use core::{intrinsics::abort, panic::PanicInfo};
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}
