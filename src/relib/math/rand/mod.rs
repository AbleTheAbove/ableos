pub mod linearshift;
pub mod prand;
pub mod wichmanhillrand; // FIXEME: Reimplement

pub trait RNG {
    fn new() -> Self;
    fn rand(&mut self) -> u64;
    fn seed(&mut self, seed: u64);
}
pub struct Random {}
