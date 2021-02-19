const CHARACTER_SET: &[u8] = b"abcdefghjkmnpqrstuwxyz0123456789";

pub fn random() -> String {
    use rand::{thread_rng, Rng};
    (0..39)
        .map(|_| CHARACTER_SET[thread_rng().gen_range(0..CHARACTER_SET.len())] as char)
        .collect()
}

pub fn hash(bytes: &[u8]) -> String {
    let mut xoodyak = xoodyak::Xoodyak::new();
    xoodyak.absorb(bytes);
    let mut buffer = [0u8; 25];
    xoodyak.squeeze_to(&mut buffer[0..24]);

    let mut result = Vec::with_capacity(40);

    for chunk in buffer.chunks_exact(5) {
        result.push(CHARACTER_SET[(chunk[0] >> 3) as usize]);
        result.push(CHARACTER_SET[(((chunk[0] << 2) & 0b11100) | ((chunk[1]) >> 6)) as usize]);
        result.push(CHARACTER_SET[((chunk[1] >> 1) & 0b11111) as usize]);
        result.push(CHARACTER_SET[(((chunk[1] << 4) & 0b10000) | (chunk[2] >> 4)) as usize]);
        result.push(CHARACTER_SET[(((chunk[2] << 1) & 0b11110) | (chunk[3] >> 7)) as usize]);
        result.push(CHARACTER_SET[((chunk[3] >> 2) & 0b11111) as usize]);
        result.push(CHARACTER_SET[(((chunk[3] << 3) & 0b11000) | (chunk[4] >> 5)) as usize]);
        result.push(CHARACTER_SET[(chunk[4] & 0b11111) as usize]);
    }

    result.truncate(39);
    String::from_utf8(result).unwrap()
}

pub fn hash_string(string: &str) -> String {
    hash(string.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_character_set() {
        assert_eq!(CHARACTER_SET.len(), 32);
        assert_eq!(
            CHARACTER_SET
                .iter()
                .cloned()
                .collect::<std::collections::HashSet<_>>()
                .len(),
            32
        );
    }

    #[test]
    fn test_random() {
        for _ in 0..128 {
            assert_eq!(random().len(), 39);
            assert!(random().bytes().all(|c| CHARACTER_SET.contains(&c)));
        }
    }

    #[test]
    fn test_hash_string() {
        assert_eq!(hash_string("hi"), "2kase65xtnxgrm4tpd8t1dscae098y1x91bc66a");

        fn random_string(count: usize) -> String {
            use rand::distributions::Alphanumeric;
            use rand::{thread_rng, Rng};

            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(count)
                .map(char::from)
                .collect()
        }

        for count in 0..128 {
            assert_eq!(hash_string(&random_string(count)).len(), 39);
            assert!(hash_string(&random_string(count))
                .bytes()
                .all(|c| CHARACTER_SET.contains(&c)));
        }
    }
}
