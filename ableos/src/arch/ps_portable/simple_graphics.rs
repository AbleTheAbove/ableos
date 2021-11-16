use crate::arch::*;
/// used as a very basic bare minimum gl example
pub fn gl_basic() {
	unsafe {
		let mut allocator = get_vram_allocator().unwrap();
		let fbp0 = allocator
			.alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm8888)
			.as_mut_ptr_from_zero();
		sys::sceGuInit();
		sys::sceGuStart(
			sys::GuContextType::Direct,
			&mut LIST as *mut _ as *mut c_void,
		);
		sys::sceGuDrawBuffer(DisplayPixelFormat::Psm8888, fbp0 as _, BUF_WIDTH as i32);
		sys::sceGuDebugPrint(
			100,
			100,
			0xff0000ff,
			b"hi there from ableOS PSP graphics\0" as *const u8,
		);
		sys::sceGuDebugFlush();
		sys::sceGuFinish();
		sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);
		sys::sceDisplayWaitVblankStart();
		sys::sceGuDisplay(true);
	}
}
