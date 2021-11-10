use crate::serial_println;

pub struct MailBoxes {
    flags: u8,
    mailboxes: [u64; 4],
}
impl MailBoxes {
    pub fn new() -> Self {
        Self {
            flags: 0b0000_0000,
            mailboxes: [0; 4],
        }
    }
    pub fn reset(&mut self) {
        self.flags = 0b0000_0000;
        self.mailboxes = [0; 4];
    }
    pub fn set_mailbox(&mut self, mailbox_num: u8, mailbox_data: u64) {
        match mailbox_num {
            0..=3 => self.mailboxes[mailbox_num as usize] = mailbox_data,
            _ => {}
        }
    }
    pub fn set_flag(&mut self, flag_num: u8) {
        match flag_num {
            0 => {
                self.flags |= 0b0000_0001;
            }
            1 => {
                self.flags |= 0b0000_0010;
            }
            2 => {
                self.flags |= 0b0000_0100;
            }
            3 => {
                self.flags |= 0b0000_1000;
            }
            4 => {
                self.flags |= 0b0001_0000;
            }
            5 => {
                self.flags |= 0b0010_0000;
            }
            6 => {
                self.flags |= 0b0100_0000;
            }
            7 => {
                self.flags |= 0b1000_0000;
            }

            _ => {}
        }
    }
    pub fn dump_flags(&self) {
        serial_println!("Flag 0: {:08b}", self.flags & 0b0000_0001);
        serial_println!("Flag 1: {:08b}", self.flags & 0b0000_0010);
        serial_println!("Flag 2: {:08b}", self.flags & 0b0000_0100);
        serial_println!("Flag 3: {:08b}", self.flags & 0b0000_1000);
        serial_println!("Flag 4: {:08b}", self.flags & 0b0001_0000);
        serial_println!("Flag 5: {:08b}", self.flags & 0b0010_0000);
        serial_println!("Flag 6: {:08b}", self.flags & 0b0100_0000);
        serial_println!("Flag 7: {:08b}", self.flags & 0b1000_0000);

        serial_println!("Flag 0: {}", self.flags >> 0 & 0b0000_0001);
        serial_println!("Flag 1: {}", self.flags >> 1 & 0b0000_0001);
        serial_println!("Flag 2: {}", self.flags >> 2 & 0b0000_0001);
        serial_println!("Flag 3: {}", self.flags >> 3 & 0b0000_0001);
        serial_println!("Flag 4: {}", self.flags >> 4 & 0b0000_0001);
        serial_println!("Flag 5: {}", self.flags >> 5 & 0b0000_0001);
        serial_println!("Flag 6: {}", self.flags >> 6 & 0b0000_0001);
        serial_println!("Flag 7: {}", self.flags >> 7 & 0b0000_0001);
    }
}
