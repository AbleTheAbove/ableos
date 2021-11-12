#[derive(Debug)]
pub struct Modifiers {
	pub lshift: bool,
	pub rshift: bool,
	pub lctrl: bool,
	pub rctrl: bool,
	pub numlock: bool,
	pub capslock: bool,
	pub alt_gr: bool,
}
impl Modifiers {
	pub fn is_shifted(&self) -> bool {
		self.lshift | self.rshift
	}
	pub fn is_ctrl(&self) -> bool {
		self.lctrl | self.rctrl
	}
	pub fn is_caps(&self) -> bool {
		self.capslock
	}
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum KeyState {
	Up,
	Down,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct KeyEvent {
	pub code: KeyCode,
	pub state: KeyState,
}
impl KeyEvent {
	pub fn new(code: KeyCode, state: KeyState) -> KeyEvent {
		KeyEvent { code, state }
	}
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum HandleControl {
	/// If either Ctrl key is held down, convert the letters A through Z into
	/// Unicode chars U+0001 through U+001A. If the Ctrl keys are not held
	/// down, letters go through normally.
	MapLettersToUnicode,
	/// Don't do anything special - send through the Ctrl key up/down events,
	/// and leave the letters as letters.
	Ignore,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DecodeState {
	Start,
	Extended,
	Release,
	ExtendedRelease,
}
/// Indicates different error conditions.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Error {
	BadStartBit,
	BadStopBit,
	ParityError,
	UnknownKeyCode,
	InvalidState,
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(u8)]
pub enum DecodedKeyKind {
	RawKey = 0,
	Unicode = 1,
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(C)]
pub struct DecodedKey {
	pub kind: DecodedKeyKind,
	pub value: u32,
}
impl From<char> for DecodedKey {
	fn from(ch: char) -> Self {
		Self {
			kind: DecodedKeyKind::Unicode,
			value: ch as u32,
		}
	}
}
impl DecodedKey {
	pub const ZERO: Self = Self {
		kind: DecodedKeyKind::Unicode,
		value: 0,
	};
	pub fn Unicode(ch: char) -> Self {
		Self {
			kind: DecodedKeyKind::Unicode,
			value: ch.into(),
		}
	}
	pub fn RawKey(byte: u8) -> Self {
		Self {
			kind: DecodedKeyKind::RawKey,
			value: byte.into(),
		}
	}
}
macro_rules! keycode_enum {
   (@get_last $Variant:ident) => {
      Self::$Variant
   };
   (@get_last $Variant:ident, $($VariantTail:ident),*) => {
      keycode_enum![@get_last $($VariantTail),*]
   };
   ($($Variant:ident=$Value:expr,)*) => {
      #[derive(Debug, PartialEq, Eq, Clone, Copy)]
      #[repr(u8)]
      pub enum KeyCode {
         $($Variant = $Value),*
      }

      impl core::convert::From<u32> for KeyCode {
         fn from(n: u32) -> Self {
            match n {
               $($Value => Self::$Variant),*,
               _ => keycode_enum![@get_last $($Variant),*]
            }
         }
      }
   };
   ($($Variant:ident=$Value:expr),* ) => {
      keycode_enum!($($Variant=$Value,)* );
   };
}
// This will be a way to map keys to other keys / keyyngs / macros
keycode_enum! {
	AltLeft = 0x00,
	AltRight = 0x01,
	ArrowDown = 0x02,
	ArrowLeft = 0x03,
	ArrowRight = 0x04,
	ArrowUp = 0x05,
	BackSlash = 0x06,
	Backspace = 0x07,
	BackTick = 0x08,
	BracketSquareLeft = 0x09,
	BracketSquareRight = 0x0A,
	CapsLock = 0x0B,
	Comma = 0x0C,
	ControlLeft = 0x0D,
	ControlRight = 0x0E,
	Delete = 0x0F,
	End = 0x10,
	Enter = 0x11,
	Escape = 0x12,
	Equals = 0x13,
	F1 = 0x14,
	F2 = 0x15,
	F3 = 0x16,
	F4 = 0x17,
	F5 = 0x18,
	F6 = 0x19,
	F7 = 0x1A,
	F8 = 0x1B,
	F9 = 0x1C,
	F10 = 0x1D,
	F11 = 0x1E,
	F12 = 0x1F,
	Fullstop = 0x20,
	Home = 0x21,
	Insert = 0x22,
	Key1 = 0x23,
	Key2 = 0x24,
	Key3 = 0x25,
	Key4 = 0x26,
	Key5 = 0x27,
	Key6 = 0x28,
	Key7 = 0x29,
	Key8 = 0x2A,
	Key9 = 0x2B,
	Key0 = 0x2C,
	Menus = 0x2D,
	Minus = 0x2E,
	Numpad0 = 0x2F,
	Numpad1 = 0x30,
	Numpad2 = 0x31,
	Numpad3 = 0x32,
	Numpad4 = 0x33,
	Numpad5 = 0x34,
	Numpad6 = 0x35,
	Numpad7 = 0x36,
	Numpad8 = 0x37,
	Numpad9 = 0x38,
	NumpadEnter = 0x39,
	NumpadLock = 0x3A,
	NumpadSlash = 0x3B,
	NumpadStar = 0x3C,
	NumpadMinus = 0x3D,
	NumpadPeriod = 0x3E,
	NumpadPlus = 0x3F,
	PageDown = 0x40,
	PageUp = 0x41,
	PauseBreak = 0x42,
	PrintScreen = 0x43,
	ScrollLock = 0x44,
	SemiColon = 0x45,
	ShiftLeft = 0x46,
	ShiftRight = 0x47,
	Slash = 0x48,
	Spacebar = 0x49,
	Tab = 0x4A,
	Quote = 0x4B,
	WindowsLeft = 0x4C,
	WindowsRight = 0x4D,
	A = 0x4E,
	B = 0x4F,
	C = 0x50,
	D = 0x51,
	E = 0x52,
	F = 0x53,
	G = 0x54,
	H = 0x55,
	I = 0x56,
	J = 0x57,
	K = 0x58,
	L = 0x59,
	M = 0x5A,
	N = 0x5B,
	O = 0x5C,
	P = 0x5D,
	Q = 0x5E,
	R = 0x5F,
	S = 0x60,
	T = 0x61,
	U = 0x62,
	V = 0x63,
	W = 0x64,
	X = 0x65,
	Y = 0x66,
	Z = 0x67,
	HashTilde = 0x68,
	PrevTrack = 0x69,
	NextTrack = 0x6A,
	Mute = 0x6B,
	Calculator = 0x6C,
	Play = 0x6D,
	Stop = 0x6E,
	VolumeDown = 0x6F,
	VolumeUp = 0x70,
	WWWHome = 0x71,
	PowerOnTestOk = 0x72,
}
