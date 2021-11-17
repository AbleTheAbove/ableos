#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {};
}
/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => {};
    ($fmt:expr) => {};
    ($fmt:expr, $($arg:tt)*) => {};
}
