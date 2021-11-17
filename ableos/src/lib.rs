#![no_std]
// #![deny(warnings)]
#![feature(asm)]
#![feature(global_asm)]
#![feature(abi_x86_interrupt)]
#![feature(core_intrinsics, lang_items, llvm_asm)]
// #![feature(alloc_error_handler)] // at the top of the file
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(crate::arch::test_runner)]
#![feature(naked_functions)]
#[cfg(target_arch = "arm")]
#[path = "arch/aarch32/mod.rs"]
mod arch;

#[cfg(target_arch = "aarch64")]
#[path = "arch/aarch64/mod.rs"]
mod arch;

#[cfg(target_arch = "x86_64")]
#[path = "arch/x86_64/mod.rs"]
mod arch;
#[cfg(target_arch = "riscv64")]
#[path = "arch/riscv/mod.rs"]
mod arch;
#[macro_use]
pub mod print;

use arch::drivers::serial;

mod driver_traits;
mod experiments;
#[cfg(not(target_arch = "mips"))]
// pub mod allocator;
mod kmain;
#[cfg(not(target_arch = "mips"))]
mod panic;
pub use experiments::server;
pub mod keyboard;
pub mod relib;
pub const KERNEL_VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(debug_assertions)]
/// A constant to check if the kernel is in debug mode
pub const RELEASE_TYPE: &str = "debug";
#[cfg(not(debug_assertions))]
/// A constant to check if the kernel is in release mode
pub const RELEASE_TYPE: &str = "release";
