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

    loop {}

    crate::arch::shutdown();
}
