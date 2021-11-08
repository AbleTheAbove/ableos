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
    for _ in 0..100000000 {
        // println!("{:?}", rand.rand());
        // println!("{:?}", rand.rand());
        // clear!();
    }
    println!("{:?}", rand.rand());

    loop {}

    crate::arch::shutdown();
}
