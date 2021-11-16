use crate::{arch::sloop, println, serial_println};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
	serial_println!("{}", info);
	sloop()
}
