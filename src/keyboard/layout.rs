use super::*;
use crate::serial_println;

pub trait KeyboardLayout {
   /// Convert a `KeyType` enum to a Unicode character, if possible.
   /// `KeyType::A` maps to `Some('a')` (or `Some('A')` if shifted), while
   /// `KeyType::AltLeft` returns `None`
   fn map_keycode(
      keycode: KeyCode,
      modifiers: &Modifiers,
      // handle_ctrl: HandleControl,
   ) -> DecodedKey;
}

pub type KeyMap = [u8; 255];

pub fn parse_format() {
   let test = include_str!("../../keymaps/qwerty.keymap").lines();
   // r#"0-NONE\n1-HI#Says HI"#
   for x in test {
      for y in x.split("-") {
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
