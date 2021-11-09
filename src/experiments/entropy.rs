pub struct Entropy {
    // Everytime entropy is used decrement bits count
    bits_count: u16,
    pool: [u8; 128],
}
impl Entropy {
    pub fn new() -> Self {
        Self {
            bits_count: 0,
            pool: [0; 128],
        }
    }
    pub fn poll_hardware() {
        todo!();
    }
    pub fn read_bit() -> u8 {
        1
    }
}
