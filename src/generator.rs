use random;
use alphabet;

pub fn universal(random: fn(usize) -> Vec<u32>, alphabet: &[char], size: usize) -> String {
    let mask = (2 << ((alphabet.len() as f64 - 1.0).ln() / 2.0_f64.ln()) as i64) - 1;
    let step: usize = (1.6_f64 * (mask * size) as f64).ceil() as usize;

    let mut id = String::new();

    'main: loop {
        let bytes = random(step);

        for i in 0..step {
            let byte: usize = bytes[i] as usize & mask;

            if alphabet.len() > byte {
                id.push(alphabet[byte]);

                if id.len() == size {
                    break 'main;
                }
            }
        }
    }

    id
}

#[cfg(test)]
mod test_universal {
    use super::*;

    #[test]
    fn generates_random_string() {
        fn random (size: usize) -> Vec<u32> {
            let sequence: Vec<u32> = vec![2, 255, 0, 1];

            let mut bytes: Vec<u32> = vec![];

            let mut i =  0;
            while i < size {
                let (elements, _) = sequence.split_at(if size - i > sequence.len() { sequence.len() } else { size - i });

                for &el in elements {
                    bytes.push(el);
                }

                i += sequence.len();
            }

            bytes
        }

        assert_eq!(universal(random, &['a', 'b', 'c'], 4), "cabc");
    }
}

pub fn fast(random: fn(usize) -> Vec<u32>, alphabet: &[char], size: usize) -> String {
    let mut id = String::new();

    let bytes = random(size);

    for i in 0..size {
        let index = bytes[i] & ((alphabet.len() as u32) - 1);

        id.push(alphabet[index as usize]);
    }

    id
}

#[cfg(test)]
mod test_fast {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn correct_length () {
        let lengths: Vec<usize> = vec![21, 5, 17, 134, 1];

        for l in lengths {
            let id = fast(random::standart, &alphabet::SAFE, l);

            assert_eq!(id.len(), l);
        }
    }

    #[test]
    fn url_friendly () {
        for _ in 0..10 {
            let id = fast(random::standart, &alphabet::SAFE, 21);

            for ch in id.chars() {
                assert!(alphabet::SAFE.contains(&ch));
            }
        }
    }

    #[test]
    fn no_collisions () {
        let count = 1_000_000;

        let mut ids = HashMap::new();

        for _ in 0..count {
            let id = fast(random::standart, &alphabet::SAFE, 21);

            if ids.contains_key(&id) {
                panic!();
            }

            ids.insert(id, true);
        }
    }

    #[test]
    fn flat_distribution () {
        let count = 1_000_000;
        let length : usize = 21;

        let mut chars = HashMap::new();

        for _ in 0..count {
            let id = fast(random::standart, &alphabet::SAFE, length);

            for ch in id.chars() {
                let counter = chars.entry(ch).or_insert(0);

                *counter += 1;
            }
        }

        for (_, &value) in &chars {
            let distribution =
                (value * alphabet::SAFE.len()) as f32 / (count as f32 * length as f32);

            assert_eq!(distribution.round(), 1.0)
        }
    }
}
