use std::{
    convert::TryInto,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{encode, Type};

pub fn time_and_hash(input: &[u8], nanoseconds_since_1970: i64) -> String {
    let mut bytes = [0u8; 25];

    let offset_binary_timestamp = (nanoseconds_since_1970 as u64) ^ (1 << 63);
    bytes[0..8].copy_from_slice(&offset_binary_timestamp.to_be_bytes());

    let mut xoodyak = xoodyak::Xoodyak::new();
    xoodyak.absorb(input);
    xoodyak.squeeze_to(&mut bytes[8..24]);

    bytes[24] = Type::TimeAndHash.id();

    encode(&bytes)
}

pub fn time_and_hash_now(input: &[u8]) -> String {
    time_and_hash(
        input,
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .try_into()
            .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_and_hash() {
        assert_eq!(
            time_and_hash("".as_bytes(), i64::MIN),
            "0000000000001tgn5wnmff729uxpdh3stjpz2yt"
        );
        assert_eq!(
            time_and_hash("".as_bytes(), 0),
            "g000000000001tgn5wnmff729uxpdh3stjpz2yt"
        );
        assert_eq!(
            time_and_hash("".as_bytes(), i64::MAX),
            "zzzzzzzzzzzzztgn5wnmff729uxpdh3stjpz2yt"
        );

        println!("{}", time_and_hash_now("".as_bytes()));
    }
}
