use alloc::alloc::{GlobalAlloc, Layout, dealloc};
use core::ptr::null_mut;

pub struct Dummy;

unsafe impl GlobalAlloc for Dummy {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, layout: Layout) {
        panic!["this implementation of dealloc should not be called"]
    }
}