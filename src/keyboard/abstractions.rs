#![allow(clippy::too_many_arguments)]
use super::*;
pub struct CustomScanCodeSet {
	single_byte: [Option<KeyCode>; 256],
	extended: [Option<KeyCode>; 256],
}
impl Default for CustomScanCodeSet {
	fn default() -> Self {
		Self::scancode_set1()
	}
}
impl CustomScanCodeSet {
	pub fn scancode_set1() -> Self {
		let mut scancode_set = Self {
			single_byte: [None; 256],
			extended: [None; 256],
		};
		scancode_set.single_byte[0x01] = Some(KeyCode::Escape); // 01
		scancode_set.single_byte[0x02] = Some(KeyCode::Key1); // 02
		scancode_set.single_byte[0x03] = Some(KeyCode::Key2); // 03
		scancode_set.single_byte[0x04] = Some(KeyCode::Key3); // 04
		scancode_set.single_byte[0x05] = Some(KeyCode::Key4); // 05
		scancode_set.single_byte[0x06] = Some(KeyCode::Key5); // 06
		scancode_set.single_byte[0x07] = Some(KeyCode::Key6); // 07
		scancode_set.single_byte[0x08] = Some(KeyCode::Key7); // 08
		scancode_set.single_byte[0x09] = Some(KeyCode::Key8); // 09
		scancode_set.single_byte[0x0A] = Some(KeyCode::Key9); // 0A
		scancode_set.single_byte[0x0B] = Some(KeyCode::Key0); // 0B
		scancode_set.single_byte[0x0C] = Some(KeyCode::Minus); // 0C
		scancode_set.single_byte[0x0D] = Some(KeyCode::Equals); // 0D
		scancode_set.single_byte[0x0E] = Some(KeyCode::Backspace); // 0E
		scancode_set.single_byte[0x0F] = Some(KeyCode::Tab); // 0F
		scancode_set.single_byte[0x10] = Some(KeyCode::Q); // 10
		scancode_set.single_byte[0x11] = Some(KeyCode::W); // 11
		scancode_set.single_byte[0x12] = Some(KeyCode::E); // 12
		scancode_set.single_byte[0x13] = Some(KeyCode::R); // 13
		scancode_set.single_byte[0x14] = Some(KeyCode::T); // 14
		scancode_set.single_byte[0x15] = Some(KeyCode::Y); // 15
		scancode_set.single_byte[0x16] = Some(KeyCode::U); // 16
		scancode_set.single_byte[0x17] = Some(KeyCode::I); // 17
		scancode_set.single_byte[0x18] = Some(KeyCode::O); // 18
		scancode_set.single_byte[0x19] = Some(KeyCode::P); // 19
		scancode_set.single_byte[0x1A] = Some(KeyCode::BracketSquareLeft); // 1A
		scancode_set.single_byte[0x1B] = Some(KeyCode::BracketSquareRight); // 1B
		scancode_set.single_byte[0x1C] = Some(KeyCode::Enter); // 1C
		scancode_set.single_byte[0x1D] = Some(KeyCode::ControlLeft); // 1D
		scancode_set.single_byte[0x1E] = Some(KeyCode::A); // 1E
		scancode_set.single_byte[0x1F] = Some(KeyCode::S); // 1F
		scancode_set.single_byte[0x20] = Some(KeyCode::D); // 20
		scancode_set.single_byte[0x21] = Some(KeyCode::F); // 21
		scancode_set.single_byte[0x22] = Some(KeyCode::G); // 22
		scancode_set.single_byte[0x23] = Some(KeyCode::H); // 23
		scancode_set.single_byte[0x24] = Some(KeyCode::J); // 24
		scancode_set.single_byte[0x25] = Some(KeyCode::K); // 25
		scancode_set.single_byte[0x26] = Some(KeyCode::L); // 26
		scancode_set.single_byte[0x27] = Some(KeyCode::SemiColon); // 27
		scancode_set.single_byte[0x28] = Some(KeyCode::Quote); // 28
		scancode_set.single_byte[0x29] = Some(KeyCode::BackTick); // 29
		scancode_set.single_byte[0x2A] = Some(KeyCode::ShiftLeft); // 2A
		scancode_set.single_byte[0x2B] = Some(KeyCode::BackSlash); // 2B
		scancode_set.single_byte[0x2C] = Some(KeyCode::Z); // 2C
		scancode_set.single_byte[0x2D] = Some(KeyCode::X); // 2D
		scancode_set.single_byte[0x2E] = Some(KeyCode::C); // 2E
		scancode_set.single_byte[0x2F] = Some(KeyCode::V); // 2F
		scancode_set.single_byte[0x30] = Some(KeyCode::B); // 30
		scancode_set.single_byte[0x31] = Some(KeyCode::N); // 31
		scancode_set.single_byte[0x32] = Some(KeyCode::M); // 32
		scancode_set.single_byte[0x33] = Some(KeyCode::Comma); // 33
		scancode_set.single_byte[0x34] = Some(KeyCode::Fullstop); // 34
		scancode_set.single_byte[0x35] = Some(KeyCode::Slash); // 35
		scancode_set.single_byte[0x36] = Some(KeyCode::ShiftRight); // 36
		scancode_set.single_byte[0x37] = Some(KeyCode::NumpadStar); // 37
		scancode_set.single_byte[0x38] = Some(KeyCode::AltLeft); // 38
		scancode_set.single_byte[0x39] = Some(KeyCode::Spacebar); // 39
		scancode_set.single_byte[0x3A] = Some(KeyCode::CapsLock); // 3A
		scancode_set.single_byte[0x3B] = Some(KeyCode::F1); // 3B
		scancode_set.single_byte[0x3C] = Some(KeyCode::F2); // 3C
		scancode_set.single_byte[0x3D] = Some(KeyCode::F3); // 3D
		scancode_set.single_byte[0x3E] = Some(KeyCode::F4); // 3E
		scancode_set.single_byte[0x3F] = Some(KeyCode::F5); // 3F
		scancode_set.single_byte[0x40] = Some(KeyCode::F6); // 40
		scancode_set.single_byte[0x41] = Some(KeyCode::F7); // 41
		scancode_set.single_byte[0x42] = Some(KeyCode::F8); // 42
		scancode_set.single_byte[0x43] = Some(KeyCode::F9); // 43
		scancode_set.single_byte[0x44] = Some(KeyCode::F10); // 44
		scancode_set.single_byte[0x45] = Some(KeyCode::NumpadLock); // 45
		scancode_set.single_byte[0x46] = Some(KeyCode::ScrollLock); // 46
		scancode_set.single_byte[0x47] = Some(KeyCode::Numpad7); // 47
		scancode_set.single_byte[0x48] = Some(KeyCode::Numpad8); // 48
		scancode_set.single_byte[0x49] = Some(KeyCode::Numpad9); // 49
		scancode_set.single_byte[0x4A] = Some(KeyCode::NumpadMinus); // 4A
		scancode_set.single_byte[0x4B] = Some(KeyCode::Numpad4); // 4B
		scancode_set.single_byte[0x4C] = Some(KeyCode::Numpad5); // 4C
		scancode_set.single_byte[0x4D] = Some(KeyCode::Numpad6); // 4D
		scancode_set.single_byte[0x4E] = Some(KeyCode::NumpadPlus); // 4E
		scancode_set.single_byte[0x4F] = Some(KeyCode::Numpad1); // 4F
		scancode_set.single_byte[0x50] = Some(KeyCode::Numpad2); // 50
		scancode_set.single_byte[0x51] = Some(KeyCode::Numpad3); // 51
		scancode_set.single_byte[0x52] = Some(KeyCode::Numpad0); // 52
		scancode_set.single_byte[0x53] = Some(KeyCode::NumpadPeriod); // 53
															  // 0x54
															  // 0x55
															  // 0x56
		scancode_set.single_byte[0x57] = Some(KeyCode::F11); // 57
		scancode_set.single_byte[0x58] = Some(KeyCode::F12); // 58
		for i in 0x81..=0xD8 {
			scancode_set.single_byte[i] = scancode_set.single_byte[i - 0x80];
		}
		scancode_set.extended[0x10] = Some(KeyCode::PrevTrack); // E010
														//0x11
														//0x12
														//0x13
														//0x14
														//0x15
														//0x16
														//0x17
														//0x18
		scancode_set.extended[0x19] = Some(KeyCode::NextTrack); // E019
														//0x1A
														//0x1B
		scancode_set.extended[0x1C] = Some(KeyCode::NumpadEnter); // E01C
		scancode_set.extended[0x1D] = Some(KeyCode::ControlRight); // E01D
														   //0x1E
														   //0x1F
		scancode_set.extended[0x20] = Some(KeyCode::Mute); // E020
		scancode_set.extended[0x21] = Some(KeyCode::Calculator); // E021
		scancode_set.extended[0x22] = Some(KeyCode::Play); // E022
												   //0x23
		scancode_set.extended[0x24] = Some(KeyCode::Stop); // E024
												   //0x25
												   //0x26
												   //0x27
												   //0x28
												   //0x29
												   //0x2A
												   //0x2B
												   //0x2C
												   //0x2D
		scancode_set.extended[0x2E] = Some(KeyCode::VolumeDown); // E02E
														 //0x2F
		scancode_set.extended[0x30] = Some(KeyCode::VolumeUp); // E030
													   //0x31
		scancode_set.extended[0x32] = Some(KeyCode::WWWHome); // E032
													  //0x33
													  //0x34
		scancode_set.extended[0x35] = Some(KeyCode::NumpadSlash); // E035
														  //0x36
														  //0x37
		scancode_set.extended[0x38] = Some(KeyCode::AltRight); // E038
													   //0x39
													   //0x3A
													   //0x3B
													   //0x3C
													   //0x3D
													   //0x3E
													   //0x3F
													   //0x40
													   //0x41
													   //0x42
													   //0x43
													   //0x44
													   //0x45
													   //0x46
		scancode_set.extended[0x47] = Some(KeyCode::Home); // E047
		scancode_set.extended[0x48] = Some(KeyCode::ArrowUp); // E048
		scancode_set.extended[0x49] = Some(KeyCode::PageUp); // E049
													 //0x4A
		scancode_set.extended[0x4B] = Some(KeyCode::ArrowLeft); // E04B
														//0x4C
		scancode_set.extended[0x4D] = Some(KeyCode::ArrowRight); // E04D
														 //0x4E
		scancode_set.extended[0x4F] = Some(KeyCode::End); // E04F
		scancode_set.extended[0x50] = Some(KeyCode::ArrowDown); // E050
		scancode_set.extended[0x51] = Some(KeyCode::PageDown); // E051
		scancode_set.extended[0x52] = Some(KeyCode::Insert); // E052
		scancode_set.extended[0x53] = Some(KeyCode::Delete); // E053
		for i in 0x90..=0xED {
			scancode_set.extended[i] = scancode_set.extended[i - 0x80];
		}
		scancode_set
	}
	pub fn scancode_set2() -> Self {
		Self {
			single_byte: [None; 256],
			extended: [None; 256],
		}
	}
}
impl ScancodeSet for CustomScanCodeSet {
	fn advance_state(&self, state: &mut DecodeState, code: u8) -> Result<Option<KeyEvent>, Error> {
		match *state {
			DecodeState::Start => {
				match code {
					EXTENDED_KEY_CODE => {
						*state = DecodeState::Extended;
						Ok(None)
					}
					0x80..=0xFF => {
						// Release codes
						Ok(Some(KeyEvent::new(
							self.map_scancode(code - 0x80)?,
							KeyState::Up,
						)))
					}
					_ => {
						// Normal codes
						Ok(Some(KeyEvent::new(
							self.map_scancode(code)?,
							KeyState::Down,
						)))
					}
				}
			}
			DecodeState::Extended => {
				*state = DecodeState::Start;
				match code {
					0x80..=0xFF => {
						// Extended Release codes
						Ok(Some(KeyEvent::new(
							self.map_extended_scancode(code - 0x80)?,
							KeyState::Up,
						)))
					}
					_ => {
						// Normal release codes
						Ok(Some(KeyEvent::new(
							self.map_extended_scancode(code)?,
							KeyState::Down,
						)))
					}
				}
			}
			_ => {
				// Can't get in to this state
				unimplemented!();
			}
		}
	}
	fn map_scancode(&self, code: u8) -> Result<KeyCode, Error> {
		if let Some(kc) = self.single_byte[code as usize] {
			Ok(kc)
		} else {
			Err(Error::UnknownKeyCode)
		}
	}
	fn map_extended_scancode(&self, code: u8) -> Result<KeyCode, Error> {
		if let Some(kc) = self.extended[code as usize] {
			Ok(kc)
		} else {
			Err(Error::UnknownKeyCode)
		}
	}
}
#[derive(Debug, Clone, Copy)]
pub enum LayoutEntry {
	Regular {
		unshifted: Option<DecodedKey>,
		shifted: Option<DecodedKey>,
		altgr: Option<DecodedKey>,
		raw_unicode: Option<DecodedKey>,
	},
	Numlockable {
		unshifted: Option<DecodedKey>,
		shifted: Option<DecodedKey>,
		locked: Option<DecodedKey>,
		locked_shifted: Option<DecodedKey>,
	},
	Capslockable {
		unshifted: Option<DecodedKey>,
		shifted: Option<DecodedKey>,
		locked: Option<DecodedKey>,
		locked_shifted: Option<DecodedKey>,
		altgr: Option<DecodedKey>,
		raw_unicode: Option<DecodedKey>,
	},
}
// Do not edit this file directly. Instead, create a `Keyboard` and modify that.
pub struct CustomLayout {
	mapping: [LayoutEntry; 256],
}
impl Default for CustomLayout {
	fn default() -> Self {
		Self::new_us104key()
	}
}
impl CustomLayout {
	#[rustfmt::skip]
	pub fn new_us104key() -> Self {
		let mut mapping = Self {
			mapping: [LayoutEntry::Regular { unshifted: None, shifted: None, altgr: None, raw_unicode: None }; 256],
		};
		mapping.set_ab_c(KeyCode::BackTick, Some('`'.into()), Some('~'.into()), None);
		mapping.set_aa_a(KeyCode::Escape, Some('\x1B'.into()));
		mapping.set_ab_n(KeyCode::Key0, Some('0'.into()), Some(')'.into()));
		mapping.set_ab_n(KeyCode::Key1, Some('1'.into()), Some('!'.into()));
		mapping.set_ab_n(KeyCode::Key2, Some('2'.into()), Some('@'.into()));
		mapping.set_ab_n(KeyCode::Key3, Some('3'.into()), Some('#'.into()));
		mapping.set_ab_n(KeyCode::Key4, Some('4'.into()), Some('$'.into()));
		mapping.set_ab_n(KeyCode::Key5, Some('5'.into()), Some('%'.into()));
		mapping.set_ab_n(KeyCode::Key6, Some('6'.into()), Some('^'.into()));
		mapping.set_ab_n(KeyCode::Key7, Some('7'.into()), Some('&'.into()));
		mapping.set_ab_n(KeyCode::Key8, Some('8'.into()), Some('*'.into()));
		mapping.set_ab_n(KeyCode::Key9, Some('9'.into()), Some('('.into()));
      mapping.set_ab_n(KeyCode::Minus, Some('-'.into()), Some('_'.into()));
      mapping.set_ab_n(KeyCode::Equals, Some('='.into()), Some('+'.into()));
      mapping.set_aa_a(KeyCode::Backspace,Some('\x08'.into()));
      mapping.set_aa_a(KeyCode::Tab,Some('\x09'.into()));
      mapping.set_abcd_e_letter(KeyCode::Q, Some('q'.into()), Some('Q'.into()), Some('Q'.into()),Some('q'.into()), Some('\u{0011}'.into()));
      mapping.set_abcd_e_letter(KeyCode::W, Some('w'.into()), Some('W'.into()), Some('W'.into()),Some('w'.into()), Some('\u{0017}'.into()));
      mapping.set_abcd_e_letter(KeyCode::E, Some('e'.into()), Some('E'.into()), Some('E'.into()),Some('e'.into()), Some('\u{0005}'.into()));
      mapping.set_abcd_e_letter(KeyCode::R, Some('r'.into()), Some('R'.into()), Some('R'.into()),Some('r'.into()), Some('\u{0012}'.into()));
      mapping.set_abcd_e_letter(KeyCode::T, Some('t'.into()), Some('T'.into()), Some('T'.into()),Some('t'.into()), Some('\u{0014}'.into()));
      mapping.set_abcd_e_letter(KeyCode::Y, Some('y'.into()), Some('Y'.into()), Some('Y'.into()),Some('y'.into()), Some('\u{0019}'.into()));
      mapping.set_abcd_e_letter(KeyCode::U, Some('u'.into()), Some('U'.into()), Some('U'.into()),Some('u'.into()), Some('\u{0015}'.into()));
      mapping.set_abcd_e_letter(KeyCode::I, Some('i'.into()), Some('I'.into()), Some('I'.into()),Some('i'.into()), Some('\u{0009}'.into()));
      mapping.set_abcd_e_letter(KeyCode::O, Some('o'.into()), Some('O'.into()), Some('O'.into()),Some('o'.into()), Some('\u{000F}'.into()));
      mapping.set_abcd_e_letter(KeyCode::P, Some('p'.into()), Some('P'.into()), Some('P'.into()),Some('p'.into()), Some('\u{0010}'.into()));
      mapping.set_abcd_e_letter(KeyCode::A, Some('a'.into()), Some('A'.into()), Some('A'.into()),Some('a'.into()), Some('\u{0001}'.into()));
      mapping.set_abcd_e_letter(KeyCode::S, Some('s'.into()), Some('S'.into()), Some('S'.into()),Some('s'.into()), Some('\u{0013}'.into()));
      mapping.set_abcd_e_letter(KeyCode::D, Some('d'.into()), Some('D'.into()), Some('D'.into()),Some('d'.into()), Some('\u{0004}'.into()));
      mapping.set_abcd_e_letter(KeyCode::F, Some('f'.into()), Some('F'.into()), Some('F'.into()),Some('f'.into()), Some('\u{0006}'.into()));
      mapping.set_abcd_e_letter(KeyCode::G, Some('g'.into()), Some('G'.into()), Some('G'.into()),Some('g'.into()), Some('\u{0007}'.into()));
      mapping.set_abcd_e_letter(KeyCode::H, Some('h'.into()), Some('H'.into()), Some('H'.into()),Some('h'.into()), Some('\u{0008}'.into()));
      mapping.set_abcd_e_letter(KeyCode::J, Some('j'.into()), Some('J'.into()), Some('J'.into()),Some('j'.into()), Some('\u{000A}'.into()));
      mapping.set_abcd_e_letter(KeyCode::K, Some('k'.into()), Some('K'.into()), Some('K'.into()),Some('k'.into()), Some('\u{000B}'.into()));
      mapping.set_abcd_e_letter(KeyCode::L, Some('l'.into()), Some('L'.into()), Some('L'.into()),Some('l'.into()), Some('\u{000C}'.into()));
      mapping.set_abcd_e_letter(KeyCode::Z, Some('z'.into()), Some('Z'.into()), Some('Z'.into()),Some('z'.into()), Some('\u{001A}'.into()));
      mapping.set_abcd_e_letter(KeyCode::X, Some('x'.into()), Some('X'.into()), Some('X'.into()),Some('x'.into()), Some('\u{0018}'.into()));
      mapping.set_abcd_e_letter(KeyCode::C, Some('c'.into()), Some('C'.into()), Some('C'.into()),Some('c'.into()), Some('\u{0003}'.into()));
      mapping.set_abcd_e_letter(KeyCode::V, Some('v'.into()), Some('V'.into()), Some('V'.into()),Some('v'.into()), Some('\u{0016}'.into()));
      mapping.set_abcd_e_letter(KeyCode::B, Some('b'.into()), Some('B'.into()), Some('B'.into()),Some('b'.into()), Some('\u{0002}'.into()));
      mapping.set_abcd_e_letter(KeyCode::N, Some('n'.into()), Some('N'.into()), Some('N'.into()),Some('n'.into()), Some('\u{000E}'.into()));
      mapping.set_abcd_e_letter(KeyCode::M, Some('m'.into()), Some('M'.into()), Some('M'.into()),Some('m'.into()), Some('\u{000D}'.into()));
      mapping.set_ab_n(KeyCode::BracketSquareLeft, Some('{'.into()), Some('['.into()));
      mapping.set_ab_n(KeyCode::BracketSquareRight, Some('}'.into()), Some(']'.into()));
      mapping.set_ab_n(KeyCode::BackSlash, Some('|'.into()), Some('\\'.into()));
      mapping.set_ab_n(KeyCode::SemiColon, Some(';'.into()), Some(':'.into()));
      mapping.set_ab_n(KeyCode::Quote, Some('\''.into()), Some('"'.into()));
      mapping.set_aa_a(KeyCode::Enter,Some('\x0A'.into()));
      mapping.set_ab_n(KeyCode::Comma, Some(','.into()), Some('<'.into()));
      mapping.set_ab_n(KeyCode::Fullstop, Some('.'.into()), Some('>'.into()));
      mapping.set_ab_n(KeyCode::Slash, Some('/'.into()), Some('?'.into()));
      mapping.set_aa_a(KeyCode::Spacebar,Some(' '.into()));
      mapping.set_aa_a(KeyCode::Delete,Some('\x7F'.into()));
      mapping.set_aaaa_num(KeyCode::NumpadSlash, Some('/'.into()), );
      mapping.set_aaaa_num(KeyCode::NumpadStar, Some('*'.into()), );
      mapping.set_aaaa_num(KeyCode::NumpadMinus, Some('-'.into()), );
      mapping.set_abba_num(KeyCode::Numpad7, Some('7'.into()), Some(DecodedKey{ kind: DecodedKeyKind::RawKey, value: KeyCode::Home as u32 }));
      mapping.set_abba_num(KeyCode::Numpad8, Some('8'.into()), Some(DecodedKey{ kind: DecodedKeyKind::RawKey, value: KeyCode::ArrowUp as u32 }));
      mapping.set_abba_num(KeyCode::Numpad9, Some('9'.into()), Some(DecodedKey{ kind: DecodedKeyKind::RawKey, value: KeyCode::PageUp as u32 }));
      mapping.set_aaaa_num(KeyCode::NumpadPlus, Some('+'.into()));
      mapping.set_abba_num(KeyCode::Numpad4, Some('4'.into()), Some(DecodedKey{ kind: DecodedKeyKind::RawKey, value: KeyCode::ArrowLeft as u32 }));
      mapping.set_aaaa_num(KeyCode::Numpad5, Some('5'.into()));
      mapping.set_abba_num(KeyCode::Numpad6, Some('6'.into()), Some(DecodedKey{ kind: DecodedKeyKind::RawKey, value: KeyCode::ArrowRight as u32 }));
      mapping.set_abba_num(KeyCode::Numpad1, Some('1'.into()), Some(DecodedKey{ kind: DecodedKeyKind::RawKey, value: KeyCode::End as u32 }));
      mapping.set_abba_num(KeyCode::Numpad2, Some('2'.into()), Some(DecodedKey{ kind: DecodedKeyKind::RawKey, value: KeyCode::ArrowDown as u32 }));
      mapping.set_abba_num(KeyCode::Numpad3, Some('3'.into()), Some(DecodedKey{ kind: DecodedKeyKind::RawKey, value: KeyCode::PageDown as u32 }));
      mapping.set_abba_num(KeyCode::Numpad0, Some('0'.into()), Some(DecodedKey{ kind: DecodedKeyKind::RawKey, value: KeyCode::Insert as u32 }));
      mapping.set_abba_num(KeyCode::NumpadPeriod, Some('.'.into()), Some('\x7F'.into()));
      mapping.set_aaaa_num(KeyCode::NumpadEnter, Some('\x0A'.into()));
		mapping
	}
	#[rustfmt::skip]
	pub fn new_us105key() -> Self {
		let mut mapping = Self::new_us104key();
		mapping.set_abcde(KeyCode::BackTick, Some('`'.into()), Some('¬'.into()), Some('|'.into()), None);
		mapping.set_ab_n(KeyCode::Key2, Some('2'.into()), Some('"'.into()));
		mapping.set_ab_n(KeyCode::Quote, Some('\''.into()), Some('@'.into()));
		mapping.set_ab_n(KeyCode::Key3, Some('3'.into()), Some('£'.into()));
		mapping.set_abcde(KeyCode::BackTick, Some('4'.into()), Some('$'.into()), Some('€'.into()), None);
		mapping.set_ab_n(KeyCode::HashTilde, Some('#'.into()), Some('~'.into()));
		mapping
	}
}
impl KeyboardLayout for CustomLayout {
	fn map_keycode(
		&self,
		keycode: KeyCode,
		modifiers: &Modifiers,
		handle_ctrl: HandleControl,
	) -> DecodedKey {
		let map_to_unicode = handle_ctrl == HandleControl::MapLettersToUnicode;
		let spot = &self.mapping[keycode as usize];
		if let Some(k) = if map_to_unicode && modifiers.is_ctrl() {
			match spot {
				LayoutEntry::Regular {
					unshifted: _,
					shifted: _,
					altgr: _,
					raw_unicode,
				} => raw_unicode,
				LayoutEntry::Numlockable {
					unshifted: _,
					shifted: _,
					locked: _,
					locked_shifted: _,
				} => &None,
				LayoutEntry::Capslockable {
					unshifted: _,
					shifted: _,
					locked: _,
					locked_shifted: _,
					raw_unicode,
					altgr: _,
				} => raw_unicode,
			}
		} else if modifiers.alt_gr {
			match spot {
				LayoutEntry::Regular {
					unshifted: _,
					shifted: _,
					altgr,
					raw_unicode: _,
				} => altgr,
				LayoutEntry::Numlockable {
					unshifted: _,
					shifted: _,
					locked: _,
					locked_shifted: _,
				} => &None,
				LayoutEntry::Capslockable {
					unshifted:_,
					shifted: _,
					locked: _,
					locked_shifted: _,
					raw_unicode: _,
					altgr,
				} => altgr,
			}
		} else if modifiers.is_shifted() {
			match spot {
				LayoutEntry::Regular {
					unshifted: _,
					shifted,
					altgr: _,
					raw_unicode: _,
				} => shifted,
				LayoutEntry::Numlockable {
					unshifted: _,
					shifted,
					locked: _,
					locked_shifted,
				} => {
					if modifiers.numlock {
						locked_shifted
					} else {
						shifted
					}
				}
				LayoutEntry::Capslockable {
					unshifted: _,
					shifted,
					locked: _,
					locked_shifted,
					raw_unicode: _,
					altgr: _,
				} => {
					if modifiers.is_caps() {
						locked_shifted
					} else {
						shifted
					}
				}
			}
		} else {
			match spot {
				LayoutEntry::Regular {
					unshifted,
					shifted: _,
					altgr: _,
					raw_unicode: _,
				} => unshifted,
				LayoutEntry::Numlockable {
					unshifted,
					shifted: _,
					locked,
					locked_shifted: _,
				} => {
					if modifiers.numlock {
						locked
					} else {
						unshifted
					}
				}
				LayoutEntry::Capslockable {
					unshifted,
					shifted: _,
					locked,
					locked_shifted: _,
					raw_unicode: _,
					altgr: _,
				} => {
					if modifiers.is_caps() {
						locked
					} else {
						unshifted
					}
				}
			}
		} {
			*k
		} else {
			DecodedKey::RawKey(keycode as u8)
		}
	}
}
// Note(elfein) Not super hard to get right, but still- DO NOT TOUCH
impl CustomLayout {
	pub fn set_abcde(
		&mut self,
		index: KeyCode,
		a: Option<DecodedKey>,
		b: Option<DecodedKey>,
		c: Option<DecodedKey>,
		d: Option<DecodedKey>,
	) {
		self.mapping[index as usize] = {
			LayoutEntry::Regular {
				unshifted: a,
				shifted: b,
				altgr: c,
				raw_unicode: d,
			}
		};
	}
	pub fn set_ab_c(
		&mut self,
		index: KeyCode,
		a: Option<DecodedKey>,
		b: Option<DecodedKey>,
		c: Option<DecodedKey>,
	) {
		self.mapping[index as usize] = {
			LayoutEntry::Regular {
				unshifted: a,
				shifted: b,
				altgr: None,
				raw_unicode: c,
			}
		};
	}
	pub fn set_ab_n(&mut self, index: KeyCode, a: Option<DecodedKey>, b: Option<DecodedKey>) {
		self.mapping[index as usize] = {
			LayoutEntry::Regular {
				unshifted: a,
				shifted: b,
				altgr: None,
				raw_unicode: None,
			}
		};
	}
	pub fn set_aa_a(&mut self, index: KeyCode, a: Option<DecodedKey>) {
		self.mapping[index as usize] = {
			LayoutEntry::Regular {
				unshifted: a,
				shifted: a,
				altgr: None,
				raw_unicode: a,
			}
		};
	}
	pub fn set_abcdef_letter(
		&mut self,
		index: KeyCode,
		a: Option<DecodedKey>,
		b: Option<DecodedKey>,
		c: Option<DecodedKey>,
		d: Option<DecodedKey>,
		e: Option<DecodedKey>,
		f: Option<DecodedKey>,
	) {
		self.mapping[index as usize] = {
			LayoutEntry::Capslockable {
				unshifted: a,
				shifted: b,
				altgr: c,
				locked: d,
				locked_shifted: e,
				raw_unicode: f,
			}
		};
	}
	pub fn set_abcd_e_letter(
		&mut self,
		index: KeyCode,
		a: Option<DecodedKey>,
		b: Option<DecodedKey>,
		c: Option<DecodedKey>,
		d: Option<DecodedKey>,
		e: Option<DecodedKey>,
	) {
		self.mapping[index as usize] = {
			LayoutEntry::Capslockable {
				unshifted: a,
				shifted: b,
				locked: c,
				locked_shifted: d,
				altgr: None,
				raw_unicode: e,
			}
		};
	}
	pub fn set_aaaa_num(&mut self, index: KeyCode, a: Option<DecodedKey>) {
		self.mapping[index as usize] = {
			LayoutEntry::Numlockable {
				unshifted: a,
				shifted: a,
				locked: a,
				locked_shifted: a,
			}
		};
	}
	pub fn set_abba_num(&mut self, index: KeyCode, a: Option<DecodedKey>, b: Option<DecodedKey>) {
		self.mapping[index as usize] = {
			LayoutEntry::Numlockable {
				unshifted: a,
				shifted: b,
				locked: b,
				locked_shifted: a,
			}
		};
	}
}
