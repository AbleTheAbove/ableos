mod alloc;
pub mod init;

use crate::agl::prelim_testing;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    prelim_testing();
    loop {}
}
