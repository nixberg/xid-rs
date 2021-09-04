use crate::{encode, Type};

pub fn random() -> String {
    random_using(&mut rand::thread_rng())
}

fn random_using<RNG: rand::Rng>(rng: &mut RNG) -> String {
    let mut bytes = [0u8; 25];

    rng.fill_bytes(&mut bytes[0..24]);

    bytes[24] = Type::Random.id();

    encode(&bytes)
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use super::*;

    #[test]
    fn test_random() {
        let mut rng = ChaCha8Rng::from_seed([0; 32]);
        assert_eq!(
            random_using(&mut rng),
            "7r0eybw9bx0dcztuq3m1y2d5m4p883p3std7yes"
        );
        assert_eq!(
            random_using(&mut rng),
            "30dy327fe4d1x62cw5sbj8bf86fm8mu78npnc69"
        );
    }
}
