use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// The starting location of the heap
pub const HEAP_START: usize = 0x_4444_4444_0000;
/// Total desired size of the heap
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

pub fn init_alloc() {
    let heap_start = HEAP_SIZE;
    let heap_end = HEAP_START;
    let heap_size = heap_end - heap_start;
    unsafe {
        ALLOCATOR.lock().init(heap_start, heap_size);
    }
}
