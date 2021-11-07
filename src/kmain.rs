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
fn graphics_holder() {
    let buffer = [[(0, 0, 0); 1440]; 900];
    /*
    let _graphics_buffer = agl::GraphicsBufferHandle {
        graphics_mode: agl::GModes::Vga800x600,
        buffer_pointer: agl::BuffPoint::Single,
        current_buff: buffer,
        double_buff: buffer,
        triple_buff: buffer,
    };

    */
}
