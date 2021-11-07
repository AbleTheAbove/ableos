pub struct Time {
    pub year: u16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub minutes: u16,
    pub seconds: u16,
    pub microseconds: u32,
}
impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}/{:?}/{:?} {:02}:{:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minutes, self.seconds
        )
    }
}

pub mod kilotime;
use core::fmt;
