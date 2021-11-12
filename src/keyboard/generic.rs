pub struct CustomLayout {
   mapping: [(DecodedKey, DecodedKey, DecodedKey, DecodedKey); 256],
}

impl CustomLayout {
   pub fn new_Us104Key() -> Self {
      Self {
         mapping: [(
            DecodedKey::ZERO,
            DecodedKey::ZERO,
            DecodedKey::ZERO,
            DecodedKey::ZERO,
         ); 256],
      }
   }
}

// impl KeyboardLayout for CustomLayout {
//    fn map_keycode(
//       keycode: KeyCode,
//       modifiers: &Modifiers,
//       // handle_ctrl: HandleControl,
//    ) -> DecodedKey {
//    }
// }

pub struct CustomScanCodeSet {
   single_byte: [KeyCode; 256],
   extended: [KeyCode; 256],
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
      Ok(self.single_byte[code as usize])
   }

   fn map_extended_scancode(&self, code: u8) -> Result<KeyCode, Error> {
      Ok(self.extended[code as usize])
   }
}
