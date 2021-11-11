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
                println!("NUM: {:?}", y.parse::<u64>().unwrap());
                serial_println!("NUM: {:?}", y.parse::<u64>().unwrap());
            } else if x.starts_with('#') {
                // ignore all # delimeted lines
            } else {
                println!("STR: {:?}", y);
                serial_println!("STR: {:?}", y);
                match x {
                    "NONE" => {
                        serial_println!("match");
                    }
                    "TAB" => {
                        serial_println!("match");
                    }
                    "SHIFT" => {
                        serial_println!("match");
                    }
                    "SCROLL_LOCK" => {
                        serial_println!("match");
                    }
                    "COMMA" => {
                        serial_println!("match");
                    }
                    "PERIOD" => {
                        serial_println!("match");
                    }
                    "QUOTE" => {
                        serial_println!("match");
                    }
                    "FUNCTION_1" => {
                        serial_println!("match");
                    }
                    "FUNCTION_2" => {
                        serial_println!("match");
                    }
                    "FUNCTION_3" => {
                        serial_println!("match");
                    }
                    "FUNCTION_4" => {
                        serial_println!("match");
                    }
                    "FUNCTION_5" => {
                        serial_println!("match");
                    }
                    "FUNCTION_6" => {
                        serial_println!("match");
                    }
                    "FUNCTION_7" => {
                        serial_println!("match");
                    }
                    "FUNCTION_8" => {
                        serial_println!("match");
                    }
                    "FUNCTION_9" => {
                        serial_println!("match");
                    }
                    "FUNCTION_10" => {
                        serial_println!("match");
                    }
                    "FUNCTION_11" => {
                        serial_println!("match");
                    }
                    "FUNCTION_12" => {
                        serial_println!("match");
                    }
                    "COMMA" => {
                        serial_println!("match");
                    }
                    "PERIOD" => {
                        serial_println!("match");
                    }
                    "FORWARDSLASH" => {
                        serial_println!("match");
                    }
                    "GRAVE" => {
                        serial_println!("match");
                    }
                    "BRACKET_LEFT" => {
                        serial_println!("match");
                    }
                    "BACK_SLASH" => {
                        serial_println!("match");
                    }
                    "BRACKET_RIGHT" => {
                        serial_println!("match");
                    }
                    "QUOTE" => {
                        serial_println!("match");
                    }
                    _ => {}
                }
            }
        }
    }
}
