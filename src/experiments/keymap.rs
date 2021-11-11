// This will be a way to map keys to other keys / keystrings / macros
enum KeyType {
   Char,
   // SpecialKey
   Shift,
   Control,
}

pub type KeyMap = [u8; 255];

pub trait ScancodeSet {
   /// Handles the state logic for the decoding of scan codes into key events.
   fn advance_state(state: &mut DecodeState, code: u8) -> Result<Option<KeyEvent>, Error>;

   /// Convert a Scan Code set X byte to our 'KeyCode' enum
   fn map_scancode(code: u8) -> Result<KeyCode, Error>;

   /// Convert a Scan Code Set X extended byte (prefixed E0) to our `KeyCode`
   /// enum.
   fn map_extended_scancode(code: u8) -> Result<KeyCode, Error>;
}

pub struct Layout {
   single_byte: [KeyType; 256],
   extended: [KeyType; 256],
}
