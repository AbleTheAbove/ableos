// Can be standardized
// NOTE: move the file to the src/ dir
pub struct SystemMemory {
    pub used: u64,
    pub total: u64,
}
impl core::fmt::Display for SystemMemory {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{} Bytes / {} Bytes", self.used, self.total)
    }
}
/*
pub fn format_system_info() -> core::string::String {
    let x = format!(
        "{}
OS: AbleOS
Host: ComputAble
Kernel: {}
Uptime: 0:0:0
Packages: 0
Shell: Ashell
Gpu: MIPS32 R4000 R4k
Cpu: {}
Memory: {}
",
        crate::experiments::BANNER,
        crate::experiments::kinfo::KINFO.kernel_version,
        crate::arch::ARCH,
        crate::experiments::kinfo::KINFO.memory
    );
    return x;
}
*/

pub const KERNEL_VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(debug_assertions)]
/// A constant to check if the kernel is in debug mode
pub const RELEASE_TYPE: &str = "debug";
#[cfg(not(debug_assertions))]
/// A constant to check if the kernel is in release mode
pub const RELEASE_TYPE: &str = "release";
