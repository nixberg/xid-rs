pub mod hash;
pub mod random;
pub mod time_and_hash;
pub mod time_and_random;

pub(crate) enum Type {
    Random,
    Hash,
    TimeAndRandom,
    TimeAndHash,
}

impl Type {
    const fn id(self) -> u8 {
        #[allow(clippy::unusual_byte_groupings)]
        match self {
            Self::Hash => 0b000_00000,
            Self::Random => 0b001_00000,
            Self::TimeAndHash => 0b010_00000,
            Self::TimeAndRandom => 0b011_00000,
        }
    }
}

pub(crate) fn encode(bytes: &[u8; 25]) -> String {
    const CHARACTER_SET: &[u8] = b"0123456789abcdefghjkmnpqrstuwxyz";

    let mut characters = Vec::with_capacity(40);

    for chunk in bytes.chunks_exact(5) {
        characters.push(CHARACTER_SET[(chunk[0] >> 3) as usize]);
        characters.push(CHARACTER_SET[(((chunk[0] << 2) & 0b11100) | ((chunk[1]) >> 6)) as usize]);
        characters.push(CHARACTER_SET[((chunk[1] >> 1) & 0b11111) as usize]);
        characters.push(CHARACTER_SET[(((chunk[1] << 4) & 0b10000) | (chunk[2] >> 4)) as usize]);
        characters.push(CHARACTER_SET[(((chunk[2] << 1) & 0b11110) | (chunk[3] >> 7)) as usize]);
        characters.push(CHARACTER_SET[((chunk[3] >> 2) & 0b11111) as usize]);
        characters.push(CHARACTER_SET[(((chunk[3] << 3) & 0b11000) | (chunk[4] >> 5)) as usize]);
        characters.push(CHARACTER_SET[(chunk[4] & 0b11111) as usize]);
    }

    characters.truncate(39);
    String::from_utf8(characters).unwrap()
}
