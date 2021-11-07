pub struct Stdout;
use core::fmt::Arguments;
use core::fmt::Error;

impl Stdout {
    pub fn write_fmt(&mut self, arg: Arguments<'_>) -> Result<(), Error> {
        core::fmt::Write::write_fmt(self, arg);
        Ok(())
    }
}
impl core::fmt::Write for Stdout {
    #[cfg(target_arch = "arm")]
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        use crate::arch::write;
        write(s);
        Ok(())
    }

    #[cfg(target_arch = "x86_64")]
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        use crate::print;
        print!("{}", s);
        Ok(())
    }

    #[cfg(target_arch = "mips")]
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        use psp::dprint;
        dprint!("{}", s);
        Ok(())
    }
}
#[macro_export]

macro_rules! print {
    () => {
        ::core::writeln!($crate::print::Stdout, "")
    };
    ($($tt:tt)*) => {
        ::core::write!($crate::print::Stdout, $($tt)*)
    };
}
#[macro_export]
macro_rules! println {
    () =>{
        ::core::writeln!($crate::print::Stdout, "\n")
    };
    ($($tt:tt)*) => {
        ::core::writeln!($crate::print::Stdout, $($tt)*)
    };
}
