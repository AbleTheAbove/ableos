use crate::{
    arch::{drivers::graphics::GraphicsBuffer, init},
    driver_traits::{graphics::Graphics, serial::Serial},
    relib::math::rand::{linearshift::LinearShiftRegister, prand::PRand, RAND_HANDLE, RNG},
    serial_print, serial_println,
};

use lazy_static::lazy_static;
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
    crate::keyboard::layout::parse_format();
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
// TODO: reimplement for the random handler
pub fn seed_rng() -> PRand {
    println!("Seeding PRNG");
    let mut data = TICK.lock();
    let mut rand = PRand::new();
    let seed = rand.rand();
    rand.seed(*data);
    println!("Seeded PRNG");
    rand
}

lazy_static! {
    // TODO: should have a sin wave influence contribution to entropy
    pub static ref TICK: spin::Mutex<u64> = spin::Mutex::new(0);
}

/// called by arch specific timers to tick up all kernel related functions
pub fn tick() {
    let mut data = TICK.lock();
    *data += 1;
    // serial_println!("{}", *data);
    RAND_HANDLE.lock().seed_entropy_timer(*data);
}
