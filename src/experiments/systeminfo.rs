pub struct SystemMemory {
    pub used: u64,
    pub total: u64,
}
impl core::fmt::Display for SystemMemory {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{} Bytes / {} Bytes", self.used, self.total)
    }
}

pub fn format_system_info() -> alloc::string::String {
    let x = format!(
        "{}
OS: AbleOS
Host: ComputAble
Kernel: {}
Uptime: 0:0:0
Packages: 0
Shell: Ashell
Gpu: MIPS32 R4000 R4k
Cpu: MIPS32 R4000 R4k
Memory: {}
",
        crate::experiments::BANNER,
        crate::experiments::kinfo::KINFO.kernel_version,
        crate::experiments::kinfo::KINFO.memory
    );
    return x;
}
