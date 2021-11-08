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
    println!("Initialized");

    stack_overflow();
    loop {}

    crate::arch::shutdown();
}

#[no_mangle]
#[allow(unconditional_recursion)]
pub extern "C" fn stack_overflow() -> u8 {
   stack_overflow();
   69
}