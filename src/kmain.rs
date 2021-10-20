use crate::arch::init;
#[no_mangle]
pub extern "C" fn kernel_main() {
    init::init();
    loop {}
}
