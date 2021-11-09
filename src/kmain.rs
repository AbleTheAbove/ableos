use crate::{
    arch::{drivers::graphics::GraphicsBuffer, init},
    driver_traits::graphics::Graphics,
};

use crate::relib::math::rand::prand::PRand;
// use crate::relib::math::rand::wichmanhillrand::WichmannHillRand;
use crate::relib::math::rand::RNG;
#[no_mangle]
pub extern "C" fn kernel_main() {
    init::init();

    GraphicsBuffer::draw();
    GraphicsBuffer::hide_cursor();
    GraphicsBuffer::show_cursor();
    println!("Initialized");

    let mut rand = PRand::new();
    let seed = rand.rand();
    rand.seed(seed);

    println!("Psuedo Random Number generated {:?}", rand.rand());

    // stack_overflow();

    loop {}

    crate::arch::shutdown();
}

#[no_mangle]
#[allow(unconditional_recursion)]
pub extern "C" fn stack_overflow() -> u8 {
    stack_overflow();
    69
}
