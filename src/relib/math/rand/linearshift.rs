use crate::relib::math::rand::RNG;

pub struct LinearShiftRegister {
    reg: u64,
}

// 64 bit
// non-cryptographically secure
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
        let entropy = 34; // replace with hardware entropy

        let shifted = (self.reg >> 1) ^ (self.reg >> 2) + entropy;
        let x: u64 = 2983745;
        let x123: u64 = 100000000;
        let multitude: u64 = x123.wrapping_mul(x) / x;
        let mult = shifted.wrapping_mul(multitude);
        let seeded_bit = seed / mult;

        if false {
            // crate::serial_println!("Entropy {}", entropy);
            // crate::serial_println!("Multitude {}", multitude);
            // crate::serial_println!("Seeded Bit {}", seeded_bit);
        }
        for x in 0..seeded_bit {
            self.rand();
        }
    }
}
