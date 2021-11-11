// This will be a way to map keys to other keys / keyyngs / macros
enum KeyType {
    Char(u8),
    // SpecialKey
    Shift,
    Control,
    Function1,
    Function2,
    Function3,
    Function4,
    Function5,
    Function6,
    Function7,
    Function8,
    Function9,
    Function10,
    Function11,
    Function12,
    Scroll_Lock,
    Minus,
    Comma,
    Period,
    ForwardSlash,
    Grave,
    Bracket_Left,
    Back_Slash,
    Bracket_Right,
    Quote,
}

pub type KeyMap = [u8; 255];
use crate::serial_println;
pub fn parse_format() {
    let test = include_str!("../../keymaps/qwerty.keymap").lines();
    // r#"0-NONE\n1-HI#Says HI"#
    for x in test {
        for y in x.split("-") {
            if y.parse::<u64>().is_ok() {
                // NOTE: this unwrap is ok bcause of the above check
                // println!("NUM: {:?}", y.parse::<u64>().unwrap());
                // serial_println!("NUM: {:?}", y.parse::<u64>().unwrap());
            } else if y.starts_with('#') {
                // ignore all # delimeted lines
            } else {
                // println!("STR: {:?}", y);
                // serial_println!("STR: {:?}", y);
                match y {
                    "NONE" => {}
                    "TAB" => {}
                    "SHIFT" => {}
                    "SCROLL_LOCK" => {}
                    "COMMA" => {}
                    "PERIOD" => {}
                    "QUOTE" => {}
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
