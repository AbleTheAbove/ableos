use crate::{
   arch::{drivers::graphics::GraphicsBuffer, init},
   driver_traits::graphics::Graphics,
};
#[no_mangle]
pub extern "C" fn kernel_main() {
   init::init();
   GraphicsBuffer::draw();
   GraphicsBuffer::hide_cursor();
   GraphicsBuffer::show_cursor();
   print!("Initialized");
   let mut x = unsafe {
      *(0xDEADBEEF as *mut u64)
   };

   loop {}

   crate::arch::shutdown();
}
