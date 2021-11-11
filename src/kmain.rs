use crate::{
    arch::{drivers::graphics::GraphicsBuffer, init},
    driver_traits::{graphics::Graphics, serial::Serial},
    relib::math::rand::{linearshift::LinearShiftRegister, prand::PRand, RNG},
    serial_print, serial_println,
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
        serial_println!("{}", prng.rand());
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

    let mut data = TIME.lock();
    // serial_println!("Seeding PRNG");
    let mut rand = PRand::new();
    let seed = rand.rand();
    rand.seed(*data);
    println!("Seeded PRNG");
    // serial_println!("Seeded PRNG");
    rand
}
use lazy_static::lazy_static;

lazy_static! {
    // TODO: should have a sin wave influence contribution to entropy
    pub static ref TIME: spin::Mutex<u64> = spin::Mutex::new(0);
}

/// called by arch specific timers to tick up all kernel related functions
pub fn tick() {
    // TIME.lock().0 += 1;

    let mut data = TIME.lock();
    *data += 1;
    println!("{:?}", *data);
}
