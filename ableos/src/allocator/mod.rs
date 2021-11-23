use alloc::alloc::{GlobalAlloc, Layout};
// use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

// mod dummy;
// use dummy::Dummy;

pub const HEAP_START: usize = 0x_4444_4444_0000;
/// 131072 bytes
pub const HEAP_MULTIPLIER: usize = 1024;
pub const HEAP_BASE: usize = 100;

pub const HEAP_SIZE: usize = HEAP_BASE * HEAP_MULTIPLIER;
// X86 alloc should be in arch/drivers/x86/alloc.rs

use crate::arch::drivers::allocator::Dummy;

#[global_allocator]
static ALLOCATOR: Dummy = Dummy;

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
