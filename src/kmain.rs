use crate::{
    arch::{drivers::graphics::GraphicsBuffer, init},
    driver_traits::{graphics::Graphics, serial::Serial},
    serial_println,
};

use crate::relib::math::rand::prand::PRand;
// use crate::relib::math::rand::wichmanhillrand::WichmannHillRand;
use crate::serial_print;

use crate::relib::math::rand::RNG;

#[no_mangle]
#[allow(unconditional_recursion)]
pub extern "C" fn stack_overflow() -> u8 {
    stack_overflow();
    69
}

#[no_mangle]
pub extern "C" fn kernel_main() {
    init::init();

    GraphicsBuffer::draw();
    GraphicsBuffer::hide_cursor();
    GraphicsBuffer::show_cursor();
    println!("Initialized");
    serial_print!("Initialized");
    let mut rand = PRand::new();
    let seed = rand.rand();
    rand.seed(seed);
    for _ in 0..1000 {
        // println!("{:?}", rand.rand());
        // println!("{:?}", rand.rand());
        // clear!();
        println!("{:?}", rand.rand());
    }
    serial_println!["Yooooo"];

    // stack_overflow();

    loop {}

    crate::arch::shutdown();
}

#[no_mangle]
pub extern "C" fn test_main() {
    init::init();

    GraphicsBuffer::draw();
    GraphicsBuffer::hide_cursor();
    GraphicsBuffer::show_cursor();
    println!("Initialized");

    let mut rand = PRand::new();
    let seed = rand.rand();
    rand.seed(seed);
    for _ in 0..100000000 {
        // println!("{:?}", rand.rand());
        // println!("{:?}", rand.rand());
        // clear!();
    }
    println!("{:?}", rand.rand());
    serial_println!["Yooooo"];

    // stack_overflow();

    loop {}

    crate::arch::shutdown();
}
