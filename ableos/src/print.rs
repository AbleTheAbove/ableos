pub struct Stdout;
use core::fmt::{Arguments, Error};
impl Stdout {
    pub fn write_fmt(&mut self, arg: Arguments<'_>) /*-> Result<(), Error> */
    {
        core::fmt::Write::write_fmt(self, arg);
        //   Ok(())
    }
}
impl core::fmt::Write for Stdout {
    #[cfg(target_arch = "aarch64")]
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        // Don't actually print anything yet lmao

        Ok(())
    }
    #[cfg(target_arch = "x86_64")]
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        use crate::kprint;
        // FIXME: causes issues
        kprint!("{}", s);
        Ok(())
    }
    #[cfg(target_arch = "riscv64")]
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
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
      //   ::core::writeln!($crate::print::Stdout, "\n")
        panic![];
    };
    ($($tt:tt)*) => {
        ::core::writeln!($crate::print::Stdout, $($tt)*)
      // panic![];
    };
}
