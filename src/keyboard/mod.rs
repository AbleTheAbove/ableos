#![allow(dead_code)]
mod abstractions;
mod small_types;
mod traits;
pub use abstractions::*;
pub use small_types::*;
pub use traits::*;
const KEYCODE_BITS: u8 = 11;
const EXTENDED_KEY_CODE: u8 = 0xE0;
const KEY_RELEASE_CODE: u8 = 0xF0;
#[derive(Debug)]
pub struct Keyboard<T, S>
where
    T: KeyboardLayout,
    S: ScancodeSet,
{
    register: u16,
    num_bits: u8,
    decode_state: DecodeState,
    handle_ctrl: HandleControl,
    modifiers: Modifiers,
    layout: T,
    set: S,
}
impl<T, S> Keyboard<T, S>
where
    T: KeyboardLayout + Default,
    S: ScancodeSet + Default,
{
    /// Make a new Keyboard object with the given layout.
    pub fn new(_layout: T, _set: S, handle_ctrl: HandleControl) -> Keyboard<T, S> {
        Keyboard {
            register: 0,
            num_bits: 0,
            decode_state: DecodeState::Start,
            handle_ctrl,
            modifiers: Modifiers {
                lshift: false,
                rshift: false,
                lctrl: false,
                rctrl: false,
                numlock: true,
                capslock: false,
                alt_gr: false,
            },
            layout: T::default(),
            set: S::default(),
        }
    }
    // /// Change the Ctrl key mapping.
    pub fn set_ctrl_handling(&mut self, new_value: HandleControl) {
        self.handle_ctrl = new_value;
    }
    // /// Get the current Ctrl key mapping.
    pub fn get_ctrl_handling(&self) -> HandleControl {
        self.handle_ctrl
    }
    /// Clears the bit register.
    ///
    /// Call this when there is a timeout reading data from the keyboard.
    pub fn clear(&mut self) {
        self.register = 0;
        self.num_bits = 0;
        self.decode_state = DecodeState::Start;
    }
    /// Processes a 16-bit word from the keyboard.
    ///
    /// * The start bit (0) must be in bit 0.
    /// * The data octet must be in bits 1..8, with the LSB in bit 1 and the
    ///   MSB in bit 8.
    /// * The parity bit must be in bit 9.
    /// * The stop bit (1) must be in bit 10.
    pub fn add_word(&mut self, word: u16) -> Result<Option<KeyEvent>, Error> {
        let byte = self.check_word(word)?;
        self.add_byte(byte)
    }
    /// Processes an 8-bit byte from the keyboard.
    ///
    /// We assume the start, stop and parity bits have been processed and
    /// verified.
    pub fn add_byte(&mut self, byte: u8) -> Result<Option<KeyEvent>, Error> {
        self.set.advance_state(&mut self.decode_state, byte)
    }
    /// Shift a bit into the register.
    ///
    /// Call this /or/ call `add_word` - don't call both.
    /// Until the last bit is added you get Ok(None) returned.
    pub fn add_bit(&mut self, bit: bool) -> Result<Option<KeyEvent>, Error> {
        self.register |= (bit as u16) << self.num_bits;
        self.num_bits += 1;
        if self.num_bits == KEYCODE_BITS {
            let word = self.register;
            self.register = 0;
            self.num_bits = 0;
            self.add_word(word)
        } else {
            Ok(None)
        }
    }
    /// Processes a `KeyEvent` returned from `add_bit`, `add_byte` or `add_word`
    /// and produces a decoded key.
    ///
    /// For example, the KeyEvent for pressing the '5' key on your keyboard
    /// gives a DecodedKey of unicode character '5', unless the shift key is
    /// held in which case you get the unicode character '%'.
    pub fn process_keyevent(&mut self, ev: KeyEvent) -> Option<DecodedKey> {
        match ev {
            KeyEvent {
                code: KeyCode::ShiftLeft,
                state: KeyState::Down,
            } => {
                self.modifiers.lshift = true;
                None
            }
            KeyEvent {
                code: KeyCode::ShiftRight,
                state: KeyState::Down,
            } => {
                self.modifiers.rshift = true;
                None
            }
            KeyEvent {
                code: KeyCode::ShiftLeft,
                state: KeyState::Up,
            } => {
                self.modifiers.lshift = false;
                None
            }
            KeyEvent {
                code: KeyCode::ShiftRight,
                state: KeyState::Up,
            } => {
                self.modifiers.rshift = false;
                None
            }
            KeyEvent {
                code: KeyCode::CapsLock,
                state: KeyState::Down,
            } => {
                self.modifiers.capslock = !self.modifiers.capslock;
                None
            }
            KeyEvent {
                code: KeyCode::NumpadLock,
                state: KeyState::Down,
            } => {
                self.modifiers.numlock = !self.modifiers.numlock;
                None
            }
            KeyEvent {
                code: KeyCode::ControlLeft,
                state: KeyState::Down,
            } => {
                self.modifiers.lctrl = true;
                None
            }
            KeyEvent {
                code: KeyCode::ControlLeft,
                state: KeyState::Up,
            } => {
                self.modifiers.lctrl = false;
                None
            }
            KeyEvent {
                code: KeyCode::ControlRight,
                state: KeyState::Down,
            } => {
                self.modifiers.rctrl = true;
                None
            }
            KeyEvent {
                code: KeyCode::ControlRight,
                state: KeyState::Up,
            } => {
                self.modifiers.rctrl = false;
                None
            }
            KeyEvent {
                code: KeyCode::AltRight,
                state: KeyState::Down,
            } => {
                self.modifiers.alt_gr = true;
                None
            }
            KeyEvent {
                code: KeyCode::AltRight,
                state: KeyState::Up,
            } => {
                self.modifiers.alt_gr = false;
                None
            }
            KeyEvent {
                code: c,
                state: KeyState::Down,
            } => Some(
                self.layout
                    .map_keycode(c, &self.modifiers, self.handle_ctrl),
            ),
            _ => None,
        }
    }
    fn get_bit(&self, word: u16, offset: usize) -> bool {
        ((word >> offset) & 0x0001) != 0
    }
    fn has_even_number_bits(&self, data: u8) -> bool {
        (data.count_ones() % 2) == 0
    }
    /// Check 11-bit word has 1 start bit, 1 stop bit and an odd parity bit.
    fn check_word(&self, word: u16) -> Result<u8, Error> {
        let start_bit = self.get_bit(word, 0);
        let parity_bit = self.get_bit(word, 9);
        let stop_bit = self.get_bit(word, 10);
        let data = ((word >> 1) & 0xFF) as u8;
        if start_bit {
            return Err(Error::BadStartBit);
        }
        if !stop_bit {
            return Err(Error::BadStopBit);
        }
        let need_parity = self.has_even_number_bits(data);
        // Odd parity, so these must not match
        if need_parity != parity_bit {
            return Err(Error::ParityError);
        }
        Ok(data)
    }
}
pub fn parse_format() {
    let test = include_str!("../../keymaps/qwerty.keymap").lines();
    // r#"0-NONE\n1-HI#Says HI"#
    for x in test {
        for y in x.split('-') {
            if y.parse::<u64>().is_ok() {
                todo![];
            // NOTE: this unwrap is ok bcause of the above check
            // println!("NUM: {:?}", y.parse::<u64>().unwrap());
            // serial_println!("NUM: {:?}", y.parse::<u64>().unwrap());
            } else if y.starts_with('#') {
                // ignore all # delimeted lines
            } else {
                // println!("STR: {:?}", y);
                // serial_println!("STR: {:?}", y);
                match y.trim() {
                    "NONE" => {}
                    "TAB" => {}
                    "SHIFT" => {}
                    "SCROLL_LOCK" => {}
                    "FUNCTION_1" => {}
                    "FUNCTION_2" => {}
                    "FUNCTION_3" => {}
                    "FUNCTION_4" => {}
                    "FUNCTION_5" => {}
                    "FUNCTION_6" => {}
                    "FUNCTION_7" => {}
                    "FUNCTION_8" => {}
                    "FUNCTION_9" => {}
                    "FUNCTION_10" => {}
                    "FUNCTION_11" => {}
                    "FUNCTION_12" => {}
                    "COMMA" => {}
                    "PERIOD" => {}
                    "FORWARDSLASH" => {}
                    "GRAVE" => {}
                    "BRACKET_LEFT" => {}
                    "BACK_SLASH" => {}
                    "BRACKET_RIGHT" => {}
                    "QUOTE" => {}
                    _ => {}
                }
            }
        }
    }
}
