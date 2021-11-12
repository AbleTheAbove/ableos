

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
      (self.lshift | self.rshift) ^ self.capslock
   }
}

// This will be a way to map keys to other keys / keyyngs / macros
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum KeyCode {
   AltLeft,
   AltRight,
   ArrowDown,
   ArrowLeft,
   ArrowRight,
   ArrowUp,
   BackSlash,
   Backspace,
   BackTick,
   BracketSquareLeft,
   BracketSquareRight,
   CapsLock,
   Comma,
   ControlLeft,
   ControlRight,
   Delete,
   End,
   Enter,
   Escape,
   Equals,
   F1,
   F2,
   F3,
   F4,
   F5,
   F6,
   F7,
   F8,
   F9,
   F10,
   F11,
   F12,
   Fullstop,
   Home,
   Insert,
   Key1,
   Key2,
   Key3,
   Key4,
   Key5,
   Key6,
   Key7,
   Key8,
   Key9,
   Key0,
   Menus,
   Minus,
   Numpad0,
   Numpad1,
   Numpad2,
   Numpad3,
   Numpad4,
   Numpad5,
   Numpad6,
   Numpad7,
   Numpad8,
   Numpad9,
   NumpadEnter,
   NumpadLock,
   NumpadSlash,
   NumpadStar,
   NumpadMinus,
   NumpadPeriod,
   NumpadPlus,
   PageDown,
   PageUp,
   PauseBreak,
   PrintScreen,
   ScrollLock,
   SemiColon,
   ShiftLeft,
   ShiftRight,
   Slash,
   Spacebar,
   Tab,
   Quote,
   WindowsLeft,
   WindowsRight,
   A,
   B,
   C,
   D,
   E,
   F,
   G,
   H,
   I,
   J,
   K,
   L,
   M,
   N,
   O,
   P,
   Q,
   R,
   S,
   T,
   U,
   V,
   W,
   X,
   Y,
   Z,
   HashTilde,
   PrevTrack,
   NextTrack,
   Mute,
   Calculator,
   Play,
   Stop,
   VolumeDown,
   VolumeUp,
   WWWHome,
   PowerOnTestOk,
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

// #[derive(Debug, PartialEq, Eq, Copy, Clone)]
// pub enum HandleControl {
//    /// If either Ctrl key is held down, convert the letters A through Z into
//    /// Unicode chars U+0001 through U+001A. If the Ctrl keys are not held
//    /// down, letters go through normally.
//    MapLettersToUnicode,
//    /// Don't do anything special - send through the Ctrl key up/down events,
//    /// and leave the letters as letters.
//    Ignore,
// }

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
   kind: DecodedKeyKind,
   value: u32,
}

impl DecodedKey {
   pub const ZERO: Self = Self {
      kind: DecodedKeyKind::Unicode,
      value: 0,
   };
}