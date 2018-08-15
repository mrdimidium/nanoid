use random;
use alphabet;

pub fn universal(random: fn(usize) -> Vec<u8>, alphabet: &[char], size: usize) -> String {
    let mask = (2 << ((alphabet.len() as f64 - 1.0).ln() / 2.0_f64.ln()) as i64) - 1;
    let step: usize = (1.6_f64 * (mask * size) as f64).ceil() as usize;

    let mut id = String::with_capacity(size);

    loop {
        let bytes = random(step);

        for &byte in &bytes {
            let byte = byte as usize & mask;

            if alphabet.len() > byte {
                id.push(alphabet[byte]);

                if id.len() == size {
                    return id;
                }
            }
        }
    }
}

#[cfg(test)]
mod test_universal {
    use super::*;

    #[test]
    fn generates_random_string() {
        fn random (size: usize) -> Vec<u8> {
            [2, 255, 0, 1].iter().cloned().cycle().take(size).collect()
        }

        assert_eq!(universal(random, &['a', 'b', 'c'], 4), "cabc");
    }
}

pub fn fast(random: fn(usize) -> Vec<u8>, alphabet: &[char], size: usize) -> String {
    random(size)
        .iter()
        .map(|&byte| alphabet[byte as usize % alphabet.len()])
        .collect()
}

#[cfg(test)]
mod test_fast {
    use super::*;
    use std::collections::{HashMap, HashSet};

    use {alphabet, random};

    #[test]
    fn correct_length () {
        let lengths = [21, 5, 17, 134, 1];

        for &l in &lengths {
            let id = fast(random::standard, &alphabet::SAFE, l);

            assert_eq!(id.len(), l);
        }
    }

    #[test]
    fn url_friendly () {
        for _ in 0..10 {
            let id = fast(random::standard, &alphabet::SAFE, 21);

            for ch in id.chars() {
                assert!(alphabet::SAFE.contains(&ch));
            }
        }
    }

    #[test]
    fn no_collisions () {
        let count = 1_000_000;
        let length: usize = 21;

        let mut ids = HashSet::with_capacity(count);

        for _ in 0..count {
            let id = fast(random::standard, &alphabet::SAFE, length);
            assert!(ids.insert(id));
        }
    }

    #[test]
    fn flat_distribution () {
        let count = 1_000_000;
        let length: usize = 21;

        let mut chars = HashMap::with_capacity(alphabet::SAFE.len());

        for _ in 0..count {
            let id = fast(random::standard, &alphabet::SAFE, length);

            for ch in id.chars() {
                let counter = chars.entry(ch).or_insert(0);

                *counter += 1;
            }
        }

        for value in chars.values() {
            let distribution =
                (value * alphabet::SAFE.len()) as f32 / (count as f32 * length as f32);

            assert_eq!(distribution.round(), 1.0)
        }
    }
}
