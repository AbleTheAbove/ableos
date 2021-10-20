use super::{getc, write, writec};

pub fn init() {
    write("Hello Rust Kernel world!");

    writec(getc());
}
