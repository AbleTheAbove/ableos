// This will be a way to map keys to other keys / keystrings / macros
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

pub fn parse_format() {
    let test = "0-NONE\n1-HI".split("-");
    for x in test {
        if x.parse::<u64>().is_ok() {
            // NOTE: this unwrap is ok bcause of the above check
            println!("NUM: {:?}", x.parse::<u64>().unwrap());
        } else {
            println!("STR: {:?}", x);
            match x {
                "NONE" => {}
                "TAB" => {}
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
                _ => {}
            }
        }
    }
}
