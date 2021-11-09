use crate::relib::math::rand::RNG;

pub struct LinearShiftRegister {
    reg: u64,
}

// 128 bit
impl RNG for LinearShiftRegister {
    fn new() -> Self {
        Self { reg: (1 << 63) | 1 }
    }
    fn rand(&mut self) -> u64 {
        let newbit = (self.reg >> 1) ^ (self.reg >> 2) ^ (self.reg >> 7);
        self.reg = (self.reg >> 1) | (newbit << 3);
        newbit
    }
    fn seed(&mut self, seed: u64) {
        let shifted = (self.reg >> 1) ^ (self.reg >> 2) + 34;
        let multitude = 1000000000000000;
        let mult = shifted.wrapping_mul(multitude);
        crate::serial_print!("{}", mult);
        let seeded_bit = seed / mult;
        for x in 0..seeded_bit {
            self.rand();
        }
    }
}
