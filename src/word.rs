// Represents a word in a word list. Each byte is expected to be a valid
// lowercase ASCII character, a-z.
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Word {
    pub bytes: Vec<u8>,
    pub bitfield: u32,
}

impl Word {
    pub fn new(bytes: Vec<u8>) -> Word {
        let mut bitfield = 0;
        for &b in &bytes {
            bitfield |= 1 << (b - b'a');
        }
        Word { bytes, bitfield }
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.bytes).unwrap()
    }
}