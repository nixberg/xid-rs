use std::{
    convert::TryInto,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{encode, Type};

pub fn time_and_random(nanoseconds_since_1970: i64) -> String {
    time_and_random_using(nanoseconds_since_1970, &mut rand::thread_rng())
}

pub fn time_and_hash_now() -> String {
    time_and_random(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .try_into()
            .unwrap(),
    )
}

fn time_and_random_using<RNG: rand::Rng>(nanoseconds_since_1970: i64, rng: &mut RNG) -> String {
    let mut bytes = [0u8; 25];

    let offset_binary_timestamp = (nanoseconds_since_1970 as u64) ^ (1 << 63);
    bytes[0..8].copy_from_slice(&offset_binary_timestamp.to_be_bytes());

    rng.fill_bytes(&mut bytes[8..24]);

    bytes[24] = Type::TimeAndRandom.id();

    encode(&bytes)
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use super::*;

    #[test]
    fn test_time_and_random() {
        let mut rng = ChaCha8Rng::from_seed([0; 32]);
        assert_eq!(
            time_and_random_using(i64::MIN, &mut rng),
            "0000000000000fg0xwqrjqt0tsznqe783w4tb8b"
        );
        assert_eq!(
            time_and_random_using(0, &mut rng),
            "g000000000000b441u1wx6kz7cc1qrc8xxrhm7k"
        );
        assert_eq!(
            time_and_random_using(i64::MAX, &mut rng),
            "zzzzzzzzzzzzz62cw5sbj8bf86fm8mu78npnc6b"
        );
    }
}
