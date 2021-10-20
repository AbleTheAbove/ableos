use super::{getc, mbox, write, writec};
use crate::*;

pub fn init() {
    write("booted on arm :>\n");
    loop {
        writec(getc());
    }
}
