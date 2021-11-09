use crate::{arch::sloop, kprintln};
use core::{intrinsics::abort, panic::PanicInfo};
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
   kprintln!("{}", info);

   sloop()
}
