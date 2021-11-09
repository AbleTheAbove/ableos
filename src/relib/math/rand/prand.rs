use crate::relib::math::rand::RNG;

pub struct PRand {
    next: u64,
}
impl RNG for PRand {
    fn new() -> Self {
        Self { next: 7 }
    }
    fn rand(&mut self) -> u64 {
        let internal_seed_1 = 21354;
        self.next = self.next.wrapping_mul(1103515245) + internal_seed_1;

        (self.next / 65536) % 32768
    }
    fn seed(&mut self, seed: u64) {
        self.next = seed;
    }
}
