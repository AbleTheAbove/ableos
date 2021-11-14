use crate::relib::math::rand::RNG;

pub struct WichmannHillRand {
    seed0: u16,
    seed1: u16,
    seed2: u16,
}
impl RNG for WichmannHillRand {
    fn new() -> Self {
        Self {
            seed0: 123,
            seed1: 456,
            seed2: 789,
        }
    }
    fn rand(&mut self) -> u64 {
        self.seed0 = (self.seed0.wrapping_mul(170)) % 30269;
        self.seed1 = (self.seed1.wrapping_mul(172)) % 30307;
        self.seed2 = (self.seed2.wrapping_mul(173)) % 30323;

        (self.seed0 / 30269 + self.seed1 / 30307 + self.seed2 / 30323).into()
    }
    fn seed(&mut self, seed: u64) {
        self.seed0 = (seed >> 48) as u16;
        self.seed1 = ((seed << 16) >> 48) as u16;
        self.seed2 = ((seed << 32) >> 48) as u16;
    }
}
