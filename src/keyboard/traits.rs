use super::*;
pub trait ScancodeSet {
	/// Handles the state logic for the decoding of scan codes into key events.
	fn advance_state(&self, state: &mut DecodeState, code: u8) -> Result<Option<KeyEvent>, Error>;
	/// Convert a Scan Code set X byte to our 'KeyType' enum
	fn map_scancode(&self, code: u8) -> Result<KeyCode, Error>;
	/// Convert a Scan Code Set X extended byte (prefixed E0) to our `KeyType`
	/// enum.
	fn map_extended_scancode(&self, code: u8) -> Result<KeyCode, Error>;
}
pub trait KeyboardLayout {
	/// Convert a `KeyType` enum to a Unicode character, if possible.
	/// `KeyType::A` maps to `Some('a')` (or `Some('A')` if shifted), while
	/// `KeyType::AltLeft` returns `None`
	fn map_keycode(
		&self,
		keycode: KeyCode,
		modifiers: &Modifiers,
		handle_ctrl: HandleControl,
	) -> DecodedKey;
}
