pub mod prand;
pub mod wichmanhillrand;
pub trait RNG {
    fn new() -> Self;
    fn rand(&mut self) -> u64;
    fn seed(&mut self, seed: u64);
}
