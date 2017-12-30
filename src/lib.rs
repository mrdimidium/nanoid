extern crate rand;

use rand::Rng;

const URL_SYMBOLS: [char; 64] = [
    '_', '~', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
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
mod random {
    use super::*;

    #[test]
    fn generates_random_vectors() {
        let bytes : Vec<u32> = random(5);

        assert_eq!(bytes.len(), 5);
    }
}

pub extern fn simple(size: usize) -> String {
    let mut id = String::new();

    let bytes = random(size);

    for i in 0..size {
        let index = bytes[i] & ((URL_SYMBOLS.len() as u32) - 1);

        id.push(URL_SYMBOLS[index as usize]);
    }

    id
}

#[cfg(test)]
mod simple {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn correct_length () {
        let lengths: Vec<usize> = vec![21, 5, 17, 134, 1];

        for l in lengths {
            let id = simple(l);

            assert_eq!(id.len(), l);
        }
    }

    #[test]
    fn url_friendly () {
        for _ in 0..10 {
            let id = simple(21);

            for ch in id.chars() {
                assert!(URL_SYMBOLS.contains(&ch));
            }
        }
    }

    #[test]
    fn no_collisions () {
        let count = 100 * 1000;

        let mut ids = HashMap::new();

        for _ in 0..count {
            let id = simple(21);

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
            let id = simple(length);

            for ch in id.chars() {
                let counter = chars.entry(ch).or_insert(0);

                *counter += 1;
            }
        }

        for (_, &value) in &chars {
            let distribution =
                (value * URL_SYMBOLS.len()) as f32 / (count as f32 * length as f32);

            assert_eq!(distribution.round(), 1.0)
        }
    }
}
