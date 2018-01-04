extern crate rand;

use rand::Rng;

const SAFE_ALPHABET: [char; 64] = [
    '_', '~',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
];

fn random(size: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();

    let mut result: Vec<u32> = vec![0; size];

    for i in 0..size {
        result[i] = rng.gen::<u32>();
    }

    result
}

#[cfg(test)]
mod test_random {
    use super::*;

    #[test]
    fn generates_random_vectors() {
        let bytes : Vec<u32> = random(5);

        assert_eq!(bytes.len(), 5);
    }
}

fn format(random: fn(usize) -> Vec<u32>, alphabet: &[char], size: usize) -> String {
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
mod test_format {
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

        assert_eq!(format(random, &['a', 'b', 'c'], 4), "cabc");
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
            let id = fast(random, &SAFE_ALPHABET, l);

            assert_eq!(id.len(), l);
        }
    }

    #[test]
    fn url_friendly () {
        for _ in 0..10 {
            let id = fast(random, &SAFE_ALPHABET, 21);

            for ch in id.chars() {
                assert!(SAFE_ALPHABET.contains(&ch));
            }
        }
    }

    #[test]
    fn no_collisions () {
        let count = 10^5;

        let mut ids = HashMap::new();

        for _ in 0..count {
            let id = fast(random, &SAFE_ALPHABET, 21);

            if ids.contains_key(&id) {
                panic!();
            }

            ids.insert(id, true);
        }
    }

    #[test]
    fn flat_distribution () {
        let count = 10^1000;
        let length : usize = 21;

        let mut chars = HashMap::new();

        for _ in 0..count {
            let id = fast(random, &SAFE_ALPHABET, length);

            for ch in id.chars() {
                let counter = chars.entry(ch).or_insert(0);

                *counter += 1;
            }
        }

        for (_, &value) in &chars {
            let distribution =
                (value * SAFE_ALPHABET.len()) as f32 / (count as f32 * length as f32);

            assert_eq!(distribution.round(), 1.0)
        }
    }
}

pub fn complex(size: usize, alphabet: &[char], random: fn(usize) -> Vec<u32>) -> String {
    let x = alphabet.len();

    if (x != 0) && ((x & (x - 1)) == 0) { // if x = 2^n
        fast(random, alphabet, size)
    } else {
        format(random, alphabet, size)
    }
}

pub fn custom(size: usize, alphabet: &[char]) -> String {
    complex(size, alphabet, random)
}

pub fn generate(size: usize) -> String {
    custom(size, &SAFE_ALPHABET)
}

pub fn simple() -> String {
    generate(21)
}
