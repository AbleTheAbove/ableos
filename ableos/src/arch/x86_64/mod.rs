use x86_64::{
    instructions::hlt,
    {structures::paging::Page, VirtAddr},
};

use bootloader::{entry_point, BootInfo};

pub mod drivers;
pub mod gdt;
pub mod init;
pub mod interrupts;
pub mod memory;

entry_point![start];
#[no_mangle]
pub fn start(boot_info: &'static BootInfo) -> ! {
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0xf021_f077_f065_804e) };

    crate::kmain::kernel_main();

    // sloop();
}

#[allow(unused)]
pub fn shutdown() -> ! {
    unsafe {
        cpuio::outw(0x2000, 0x604);
    }

    sloop();
}

pub fn sloop() -> ! {
    loop {
        hlt();
    }
}
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test();
    }
}
