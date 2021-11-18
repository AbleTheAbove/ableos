use x86_64::instructions::hlt;
pub mod drivers;
pub mod gdt;
pub mod init;
pub mod interrupts;
#[no_mangle]
pub extern "C" fn _start() -> ! {
    crate::kmain::kernel_main();
    sloop();
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