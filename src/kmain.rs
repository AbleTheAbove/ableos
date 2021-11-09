use crate::{
    arch::init,
    driver_traits::{graphics::Graphics, serial::Serial},
    // serial_print, serial_println,
    graphics::GraphicsBuffer,
    relib::math::rand::{linearshift::LinearShiftRegister, prand::PRand, RNG},
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

    GraphicsBuffer::draw();
    GraphicsBuffer::hide_cursor();
    GraphicsBuffer::show_cursor();
    {
        let mut prng = seed_rng();
        prng.rand();
        println!("{}", prng.rand());
    }

    {
        use crate::experiments::mail::MailBoxes;
        let mut x = MailBoxes::new();
        x.set_flag(1);
        x.set_flag(2);
        // x.dump_flags();
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
