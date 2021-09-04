use crate::{encode, Type};

pub fn hash(input: &[u8]) -> String {
    let mut bytes = [0u8; 25];

    let mut xoodyak = xoodyak::Xoodyak::new();
    xoodyak.absorb(input);
    xoodyak.squeeze_to(&mut bytes[0..24]);

    bytes[24] = Type::Hash.id();

    encode(&bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(
            hash("".as_bytes()),
            "x8ajyat7qkh4xyu6rhwx9bfhff9j9p06x1fzeqg"
        );
    }
}
