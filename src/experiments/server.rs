pub trait Server {
    /// Initialize the server and return a number
    fn initialize() -> u32;
    /// kill the server
    fn kill();
}
