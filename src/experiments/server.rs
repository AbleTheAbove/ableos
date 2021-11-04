pub trait Server {
    /// Initialize the server and return a number
    fn initialize() -> u32;
    /// kill the server
    fn kill();

    // put data in the servers outbox
    fn send();

    // put data in the servers inbox and notify it
    fn recieve();
}
