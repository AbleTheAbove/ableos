#![allow(dead_code)]
pub mod linearshift;
pub mod prand;
pub mod wichmanhillrand; // FIXEME: Reimplement

use crate::serial_println;
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
    pool: [u8; 255],
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
    key_handle: KeyEntropyHandler,
}
impl RandomHandeler {
    pub fn seed_entropy(&mut self) {}
    // FIXME: Likely to panic

    pub fn seed_entropy_keyboard(&mut self, key: u8) {
        crate::serial_println!("{}", self.key_handle);
        if self.key_handle > 7 {
            self.key_handle = 0
        }
        self.entropy.pool[self.key_handle as usize] += key;
        self.key_handle += 1;
    }
    pub fn seed_entropy_timer(&mut self, seed: u64) {
        let bytes = seed.to_be_bytes();
        serial_println!("{:?}", bytes);

        for byte in bytes {
            self.entropy.pool[self.entropy.pool_index as usize] =
                self.entropy.pool[self.entropy.pool_index as usize].wrapping_mul(byte);
            self.entropy.pool_index.wrapping_add(1);
        }
    }
}

lazy_static! {
    pub static ref RAND_HANDLE: spin::Mutex<RandomHandeler> = spin::Mutex::new(RandomHandeler {
        prand: PRand::new(),
        linearshift: LinearShiftRegister::new(),
        entropy: Entropy::new(),
        key_handle: 0,
    });
}
