pub const ARCH: &'static str = "x86_64";

use x86_64::instructions::hlt;
pub mod drivers;
pub mod gdt;
pub mod init;
pub mod interrupts;

#[no_mangle]
pub extern "C" fn _start() -> ! {
   crate::kmain::kernel_main();
   sloop();
}

pub fn shutdown() -> ! {
   sloop();
}
pub fn sloop() -> ! {
   loop {
      hlt();
   }
}
