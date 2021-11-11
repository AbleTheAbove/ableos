#[repr(C)]
pub struct ShutterDowner {
   smi_cmd: u32,
   acpi_enable: u8,
   acpi_disable: u8,
   pm1a_cnt: u32,
   pm1b_cnt: u32,
   slp_typa: u16,
   slp_typb: u16,
   slp_en: u16,
   scii_en: u16,
   pm1_cnt_len: u8,
}

pub structRSDPtr {
   signature: [u8; 8],
   checksum: u8,
   oem_id: [u8; 6],
   revision: u8,
   rsdt_address: u32,
}

struct FACP {
   signature: [u8; 4],
   length: u32,
   
}