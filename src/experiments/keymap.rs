// This will be a way to map keys to other keys / keystrings / macros
enum KeyType {
    Char,
    // SpecialKey
    Shift,
    Control,
}

pub type KeyMap = [u8; 255];
