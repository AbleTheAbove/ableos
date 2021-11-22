//! hi

#![no_std]
#![feature(
    abi_x86_interrupt,
    asm,
    core_intrinsics,
    global_asm,
    lang_items,
    llvm_asm,
    naked_functions
)]

#[cfg(target_arch = "aarch64")]
#[path = "arch/aarch64/mod.rs"]
pub mod arch;

#[cfg(target_arch = "x86_64")]
#[path = "arch/x86_64/mod.rs"]
pub mod arch;

#[cfg(target_arch = "riscv64")]
#[path = "arch/riscv/mod.rs"]
pub mod arch;

#[macro_use]
pub mod print;

use arch::drivers::serial;

pub mod driver_traits;
pub mod experiments;
pub mod keyboard;
pub mod kmain;
pub mod panic;
pub mod relib;

use experiments::server;
