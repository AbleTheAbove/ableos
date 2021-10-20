pub mod init;
use cpuio::outb;

static HELLO: &[u8] = b"Running on x86_64";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    disable_cursor();

    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}

fn disable_cursor() {
    unsafe {
        outb(0x0A, 0x3D4);
        outb(0x20, 0x3D5);
    }
}
