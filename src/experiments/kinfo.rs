pub struct SemanticVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl core::fmt::Display for SemanticVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "v{}.{}.{}", self.major, self.minor, self.patch)
    }
}

lazy_static! {
    pub static ref KINFO: KernelInfo = KernelInfo {
        kernel_version: SemanticVersion {
            major: 0,
            minor: 0,
            patch: 0,
        },
        memory: SystemMemory { used: 0, total: 0 }
    };
}

/// simple info you would want to know in a neofetch like program
pub struct KernelInfo {
    // os: String,
    // host: String,
    pub kernel_version: SemanticVersion,
    // cpu: String,
    // gpu: String,
    pub memory: SystemMemory,
}
use super::systeminfo::SystemMemory;
use lazy_static::lazy_static;
