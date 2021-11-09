use crate::{
    arch::init,
    driver_traits::{graphics::Graphics, serial::Serial},
    relib::math::rand::{linearshift::LinearShiftRegister, prand::PRand, RNG},
    // serial_print, serial_println,
};

#[no_mangle]
#[allow(unconditional_recursion)]
pub extern "C" fn stack_overflow() -> u8 {
    stack_overflow();
    69 // NOTE: Any specific reason for this number asside from memes?
}

#[no_mangle]
pub extern "C" fn kernel_main() {
    init::init();

    // GraphicsBuffer::draw();
    // GraphicsBuffer::hide_cursor();
    // GraphicsBuffer::show_cursor();
    {
        let mut prng = seed_rng();
        prng.rand();
        println!("{}", prng.rand());
    }

    // stack_overflow();

    loop {}

    crate::arch::shutdown();
}

pub fn seed_rng() -> PRand {
    println!("Seeding PRNG");
    // serial_println!("Seeding PRNG");
    let mut rand = PRand::new();
    let seed = rand.rand();
    rand.seed(seed);
    println!("Seeded PRNG");
    // serial_println!("Seeded PRNG");
    rand
}

/*
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
*/
