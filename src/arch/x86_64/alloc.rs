use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// The starting location of the heap
pub const HEAP_START: usize = 0x_4444_4444_0000;
/// Total desired size of the heap
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

pub fn init_alloc() {}
