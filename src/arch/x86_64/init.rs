use super::alloc::init_alloc;
pub fn init() {
    init_alloc();

    let vga_buffer = 0xb8000 as *mut u8;
    let hello: &[u8] = b"Running on x86_64";

    for (i, &byte) in hello.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}
