use super::Time;
use core::fmt::{Display, Error, Formatter};
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Kilosecond(usize);
impl Display for Kilosecond {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		let mut reg = self.0;
		let mut buf = [0u8, 0, 0, 0, 0];
		let mut n = 0;
		while n < 8 {
			if n > 2 {
				buf[7 - n] = match reg % 10 {
					0 => b'0',
					1 => b'1',
					2 => b'2',
					3 => b'3',
					4 => b'4',
					5 => b'5',
					6 => b'6',
					7 => b'7',
					8 => b'8',
					9 => b'9',
					_ => unreachable!["CPU is borken"],
				};
			}
			reg /= 10;
			n += 1;
		}
		for (n, b) in buf.iter().enumerate() {
			write![f, "{}", *b as char]?;
			if n == 1 {
				write![f, "."]?;
			}
		}
		Ok(())
	}
}
impl core::ops::Add for Kilosecond {
	type Output = Self;
	fn add(self, rhs: Self) -> Self {
		Self(self.0 + rhs.0)
	}
}
impl core::ops::Sub for Kilosecond {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self {
		Self(self.0 - rhs.0)
	}
}
impl From<Time> for Kilosecond {
	fn from(t: Time) -> Self {
		Self((t.hour as usize * 3600 + t.minutes as usize * 60 + t.seconds as usize) * 1000)
	}
}
impl Kilosecond {
	pub fn from_ms(ms: usize) -> Self {
		Self(ms)
	}
	pub fn from_sec(sec: usize) -> Self {
		Self(sec * 1000)
	}
	pub fn from_minutes(min: usize) -> Self {
		Self(min * 60 * 1000)
	}
	pub fn from_hours(hrs: usize) -> Self {
		Self(hrs * 60 * 60 * 1000)
	}
	pub fn from_days(days: usize) -> Self {
		Self(days * 24 * 60 * 60 * 1000)
	}
}
