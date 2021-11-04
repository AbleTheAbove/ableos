#![no_std]
// #![deny(warnings)]
#![feature(core_intrinsics, lang_items, llvm_asm)]
#![feature(alloc_error_handler)] // at the top of the file

#[cfg(target_arch = "arm")]
#[path = "arch/aarch32/mod.rs"]
mod arch;

#[cfg(target_arch = "x86_64")]
#[path = "arch/x86_64/mod.rs"]
mod arch;

#[cfg(target_arch = "mips")]
#[path = "arch/ps_portable/mod.rs"]
mod arch;

#[macro_use]
pub extern crate alloc;

#[cfg(not(target_arch = "mips"))]
pub mod allocator;
mod kmain;

#[cfg(not(target_arch = "mips"))]
mod panic;
use able_graphics_lib as agl;

mod experiments;
pub use experiments::server;

pub const KERNEL_VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(debug_assertions)]
/// A constant to check if the kernel is in debug mode
pub const RELEASE_TYPE: &str = "debug";
#[cfg(not(debug_assertions))]
/// A constant to check if the kernel is in release mode
pub const RELEASE_TYPE: &str = "release";
mod time;
