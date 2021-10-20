#![no_std]
#![feature(core_intrinsics, lang_items, llvm_asm)]
#![feature(alloc_error_handler)] // at the top of the file

#[cfg(target_arch = "arm")]
#[path = "arch/arm/mod.rs"]
mod arch;

#[cfg(target_arch = "x86_64")]
#[path = "arch/x86_64/mod.rs"]
mod arch;

mod kmain;
mod panic;

pub const KERNEL_VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(debug_assertions)]
/// A constant to check if the kernel is in debug mode
pub const RELEASE_TYPE: &str = "debug";
#[cfg(not(debug_assertions))]
/// A constant to check if the kernel is in release mode
pub const RELEASE_TYPE: &str = "release";

// These functions below provide definitions for symbols libcore
// expects which are not present on our bare metal target. You
// will not encounter linker errors until you use a part of
// libcore that references them, such as iterators in this program.
// In the future you may need to provide real implementations for
// these functions.

use able_graphics_lib as agl;

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {}
#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() {
    loop {}
}
