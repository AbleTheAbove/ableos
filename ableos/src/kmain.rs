#![allow(clippy::empty_loop)]
use crate::{
	arch::{
		drivers::graphics::GraphicsBuffer,
		init,
		memory::{self, translate_addr},
		sloop,
	},
	driver_traits::{graphics::Graphics, serial::Serial},
	relib::math::rand::{linearshift::LinearShiftRegister, prand::PRand, RAND_HANDLE, RNG},
	serial_print, serial_println,
};
use bootloader::{entry_point, BootInfo};
use lazy_static::lazy_static;
use x86_64::{VirtAddr, structures::paging::Page};
#[no_mangle]
#[allow(unconditional_recursion)]
pub extern "C" fn stack_overflow() -> u8 {
	stack_overflow();
	// meme number
	69 // NOTE: Any specific reason for this number asside from memes?
}

use crate::keyboard::DecodedKey;

lazy_static! {
	pub static ref KEY_BUFFER: [DecodedKey; 256] = [DecodedKey::RawKey(123); 256];
	pub static ref KEY_BUFFER_POINTER: u8 = 0;
}
// Defines the entry point
entry_point![kernel_main];
#[no_mangle]
pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
	init::init();

	GraphicsBuffer::draw();
	GraphicsBuffer::hide_cursor();
	GraphicsBuffer::show_cursor();
	seed_rng();

	/* If AES is present then AES init rng as well
	// Maybe via a cfg
		AES::init_rng();

		*/
	#[cfg(not(target_arch = "riscv64"))]
	println!("init");

	{
		use crate::experiments::mail::MailBoxes;
		let mut x = MailBoxes::new();
		x.set_flag(1);
		x.set_flag(2);
		// x.dump_flags();
	}

	let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

	let mut mapper = unsafe { memory::init(phys_mem_offset) };
	let mut frame_allocator = memory::EmptyFrameAllocator;

	let page = Page::containing_address(VirtAddr::new(0));
	memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

	let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
	unsafe { page_ptr.offset(400).write_volatile(0xf021_f077_f065_f04e) };

	// stack_overflow();
	// crate::arch::shutdown();
	sloop()
}
// TODO: reimplement for the random handler
pub fn seed_rng() -> PRand {
	println!("Seeding PRNG");
	let data = TICK.lock();
	let mut rand = PRand::new();
	let seed = rand.rand();
	println!("{:?}", seed);
	rand.seed(*data);
	println!("Seeded PRNG");
	rand
}
lazy_static! {
	// TODO: should have a sin wave influence contribution to entropy
	pub static ref TICK: spin::Mutex<u64> = spin::Mutex::new(0);
}
/// called by arch specific timers to tick up all kernel related functions
pub fn tick() {
	let mut data = TICK.lock();
	*data += 1;
	// serial_println!("{}", *data);
	RAND_HANDLE.lock().seed_entropy_timer(*data);
}
pub fn key_entropy(key: u8) {
	RAND_HANDLE.lock().seed_entropy_keyboard(key);
}
