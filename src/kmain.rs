use crate::arch::init;

#[no_mangle]
pub extern "C" fn kernel_main() {
    // graphics_holder();
    init::init();
    loop {}
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
