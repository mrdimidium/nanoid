extern crate rand;

pub mod url;
pub mod random;

pub extern fn simple(size: usize) -> String {
    let mut id = String::new();

    let bytes = random::gen(size);

    for i in 0..size {
        let index = bytes[i] & ((url::SYMBOLS.len() as u32) - 1);

        id.push(url::SYMBOLS[index as usize]);
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
                assert!(url::SYMBOLS.contains(&ch));
            }
        }
    }

    #[test]
    fn no_collisions () {
        let count = 10^5;

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
                (value * url::SYMBOLS.len()) as f32 / (count as f32 * length as f32);

            assert_eq!(distribution.round(), 1.0)
        }
    }
}
