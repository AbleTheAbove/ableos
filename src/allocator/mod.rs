/// Wrapper around Mutex
pub struct Locked<A> {
    inner: spin::Mutex<A>,
}
impl<A> Locked<A> {
    /// Creates a new Locked spin mutex
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }
    /// Locks the spin mutex
    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
