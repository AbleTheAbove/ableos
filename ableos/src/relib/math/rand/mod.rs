#![allow(dead_code)]
pub mod linearshift;
pub mod prand;
pub mod wichmanhillrand; // FIXEME: Reimplement
use lazy_static::lazy_static;
use linearshift::LinearShiftRegister;
use prand::PRand;
pub trait RNG {
    fn new() -> Self;
    fn rand(&mut self) -> u64;
    fn seed(&mut self, seed: u64);
}

pub type KeyEntropyHandler = u8;

pub struct Entropy {
    // Everytime entropy is used decrement bits count
    bytes_count: u8, // 167 is our lower desired bit count
    pool_index: u8,
    pool: [u64; 255],
}
impl Entropy {
    pub fn new() -> Self {
        Self {
            bytes_count: 0,
            pool: [0; 255],
            pool_index: 0,
        }
    }
    pub fn poll_hardware() {
        todo!();
    }
    pub fn read_entropy(&mut self) -> u8 {
        self.bytes_count -= 1;
        1
    }
}
impl Default for Entropy {
    fn default() -> Self {
        Self::new()
    }
}
pub struct RandomHandeler {
    prand: prand::PRand,
    linearshift: linearshift::LinearShiftRegister,
    entropy: Entropy,
}
impl RandomHandeler {
    pub fn seed_entropy(&mut self) {
        // n is even
        self.prand
            .seed(self.entropy.pool[self.entropy.pool_index as usize]);
        //otherwise odd
        self.linearshift
            .seed(self.entropy.pool[self.entropy.pool_index as usize]);
    }
    // FIXME: Likely to panic

    pub fn seed_entropy_keyboard(&mut self, key: u8) {
        self.entropy.pool_index += key;
        if self.entropy.pool_index > 254 {
            self.entropy.pool_index = 0
        }
        self.entropy.pool[self.entropy.pool_index as usize] += key as u64;

        self.entropy.pool_index += 1;
    }
    pub fn seed_entropy_timer(&mut self, seed: u64) {
        let bytes = seed.to_be_bytes();

        for byte in bytes {
            if self.entropy.pool_index > 254 {
                self.entropy.pool_index = 0
            }
            self.entropy.pool[self.entropy.pool_index as usize] += byte as u64;
            self.entropy.pool_index += 1;
        }
    }
}
lazy_static! {
    pub static ref RAND_HANDLE: spin::Mutex<RandomHandeler> = spin::Mutex::new(RandomHandeler {
        prand: PRand::new(),
        linearshift: LinearShiftRegister::new(),
        entropy: Entropy::new(),
    });
}
