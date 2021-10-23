mod alloc;
pub mod init;

use crate::agl::{disable_cursor, prelim_testing};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    disable_cursor();
    prelim_testing();
    loop {}
}
