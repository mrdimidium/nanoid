//! A tiny, secure, URL-friendly, unique string ID generator
//!
//! **Safe.** It uses cryptographically strong random APIs
//! and guarantees a proper distribution of symbols.
//!
//! **Compact.** It uses a larger alphabet than UUID (`A-Za-z0-9_-`)
//! and has a similar number of unique IDs in just 21 symbols instead of 36.
//!
//! ```toml
//! [dependencies]
//! nanoid = "0.4.0"
//! ```
//!
//! ```rust
//! use nanoid::nanoid;
//!
//! let id = nanoid!(); //=> "Yo1Tr9F3iF-LFHX9i9GvA"
//! # assert_eq!(id.len(), 21);
//! ```
//!
//! ## Usage
//!
//! ### Simple
//!
//! The main module uses URL-friendly symbols (`A-Za-z0-9_-`) and returns an ID
//! with 21 characters.
//!
//! ```rust
//! use nanoid::nanoid;
//!
//! let id = nanoid!(); //=> "Yo1Tr9F3iF-LFHX9i9GvA"
//! # assert_eq!(id.len(), 21);
//! ```
//!
//! Symbols `-,.()` are not encoded in the URL. If used at the end of a link
//! they could be identified as a punctuation symbol.
//!
//! ### Custom length
//!
//! If you want to reduce ID length (and increase collisions probability),
//! you can pass the length as an argument generate function:
//!
//! ```rust
//! use nanoid::nanoid;
//!
//! let id = nanoid!(10); //=> "IRFa-VaY2b"
//! # assert_eq!(id.len(), 10);
//! ```
//!
//! ### Custom Alphabet or Length
//!
//! If you want to change the ID's alphabet or length, you can pass
//! a custom alphabet to the `nanoid!()` macro as the second parameter.
//!
//! ```rust
//! use nanoid::nanoid;
//!
//! let alphabet: [char; 16] = [
//!     '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f'
//! ];
//!
//! let id = nanoid!(10, &alphabet); //=> "4f90d13a42"
//! # assert_eq!(id.len(), 10);
//! ```
//!
//! Alphabet must contain 256 symbols or less.
//! Otherwise, the generator will not be secure.
//!
//! ### Custom Random Bytes Generator
//!
//! You can replace the default safe random generator by passing your own
//! function as the third argument to `nanoid!()`. For instance, to use a
//! seed-based generator.
//!
//! ```rust
//! use nanoid::nanoid;
//!
//! fn random_byte () -> u8 {
//!     0
//! }
//!
//! fn main() {
//!     fn random (size: usize) -> Vec<u8> {
//!         let mut bytes: Vec<u8> = vec![0; size];
//!
//!         for i in 0..size {
//!             bytes[i] = random_byte();
//!         }
//!
//!         bytes
//!     }
//!
//!     nanoid!(10, &['a', 'b', 'c', 'd', 'e', 'f'], random); //=> "fbaefaadeb"
//! }
//! ```
//!
//! `random` function must accept the array size and return an vector
//! with random numbers.
//!
//! If you want to use the same URL-friendly symbols with a custom random
//! source, the default alphabet is exposed as `nanoid::alphabet::SAFE`:
//!
//! ```rust
//! use nanoid::nanoid;
//!
//! fn random (size: usize) -> Vec<u8> {
//!     let result: Vec<u8> = vec![0; size];
//!
//!     result
//! }
//!
//! fn main() {
//!     nanoid!(10, &nanoid::alphabet::SAFE, random); //=> "93ce_Ltuub"
//! }
//! ```
//!
//! ### Seeded Random Generator
//!
//! You can use a seeded random generator for reproducible IDs.
//! This is useful for testing or when you need deterministic output.
//!
//! ```rust
//! use nanoid::nanoid;
//! use rand::{rngs::StdRng, Rng, SeedableRng};
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! let id = nanoid!(10, &nanoid::alphabet::SAFE, |size| {
//!     let mut bytes = vec![0u8; size];
//!     rng.fill(&mut bytes[..]);
//!     bytes
//! });
//!
//! # assert_eq!(id.len(), 10);
//! println!("{}", id); //=> "wyBwxRa4Xf"
//! ```
//!
//! The random generator accepts `Fn` and `FnMut` closures, allowing you to use
//! stateful random generators. This enables use cases like:
//!
//! - Seeded RNGs for reproducible IDs
//! - Custom stateful generators
//! - Integration with external random sources
//!

#![doc(
    html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png",
    html_favicon_url = "https://www.rust-lang.org/favicon.ico",
    html_root_url = "https://docs.rs/nanoid"
)]

#[cfg(feature = "smartstring")]
use smartstring::alias::String;

pub mod alphabet;
pub mod rngs;

pub fn format<F: FnMut(usize) -> Vec<u8>>(random: F, alphabet: &[char], size: usize) -> String {
    assert!(
        alphabet.len() <= u8::MAX as usize,
        "The alphabet cannot be longer than a `u8` (to comply with the `random` function)"
    );

    #[cfg(not(feature = "smartstring"))]
    let mut id = String::with_capacity(size);
    #[cfg(feature = "smartstring")]
    let mut id = String::new();

    if alphabet.len().is_power_of_two() {
        fast_impl(&mut id, random, alphabet, size);
    } else {
        generic_impl(&mut id, random, alphabet, size);
    }
    id
}

/// Generic implementation that works for any alphabet with up to 256 characters.
fn generic_impl<F: FnMut(usize) -> Vec<u8>>(
    id: &mut String,
    mut random: F,
    alphabet: &[char],
    size: usize,
) {
    let mask = alphabet.len().next_power_of_two() - 1;
    let step: usize = 8 * size / 5;

    // Assert that the masking does not truncate the alphabet. (See #9)
    debug_assert!(alphabet.len() <= mask + 1);

    loop {
        let bytes = random(step);

        for &byte in &bytes {
            let byte = byte as usize & mask;

            if alphabet.len() > byte {
                id.push(alphabet[byte]);

                if id.len() == size {
                    return;
                }
            }
        }
    }
}

/// Faster implementation that assumes the size of the alphabet is a power of 2.
///
/// This allows us to skip some checks and branches that are necessary in the general case.
fn fast_impl<F: FnMut(usize) -> Vec<u8>>(
    id: &mut String,
    mut random: F,
    alphabet: &[char],
    size: usize,
) {
    debug_assert!(alphabet.len().is_power_of_two());

    let mask = alphabet.len() - 1;

    // Since we never discard values, we can request the exact number of bytes up front.
    let bytes = random(size);

    for &byte in &bytes {
        let byte = byte as usize & mask;
        id.push(alphabet[byte]);
    }
}

#[cfg(test)]
mod test_format {
    use super::*;

    #[test]
    fn generates_random_string() {
        fn random(size: usize) -> Vec<u8> {
            [2, 255, 0, 1].iter().cloned().cycle().take(size).collect()
        }

        assert_eq!(format(random, &['a', 'b', 'c'], 4), "cabc");
    }

    #[test]
    #[should_panic]
    fn bad_alphabet() {
        let alphabet: Vec<char> = (0..32_u8).cycle().map(|i| i as char).take(1000).collect();
        nanoid!(21, &alphabet);
    }

    #[test]
    fn non_power_2() {
        let id: String = nanoid!(42, &alphabet::SAFE[0..62]);

        assert_eq!(id.len(), 42);
    }

    #[test]
    fn power_of_two_uses_fast_path() {
        fn random(size: usize) -> Vec<u8> {
            (0..size as u8).collect()
        }

        let alphabet: [char; 4] = ['a', 'b', 'c', 'd'];
        // With a power-of-two alphabet, every byte maps directly via `byte & mask`,
        // so the output is fully determined by the input bytes.
        assert_eq!(format(random, &alphabet, 8), "abcdabcd");
    }
}

#[macro_export]
macro_rules! nanoid {
    // simple
    () => {
        $crate::format($crate::rngs::default, &$crate::alphabet::SAFE, 21)
    };

    // generate
    ($size:expr) => {
        $crate::format($crate::rngs::default, &$crate::alphabet::SAFE, $size)
    };

    // custom
    ($size:expr, $alphabet:expr) => {
        $crate::format($crate::rngs::default, $alphabet, $size)
    };

    // complex
    ($size:expr, $alphabet:expr, $random:expr) => {
        $crate::format($random, $alphabet, $size)
    };
}

#[cfg(test)]
mod test_macros {
    use super::*;

    #[test]
    fn simple() {
        let id: String = nanoid!();

        assert_eq!(id.len(), 21);
    }

    #[test]
    fn generate() {
        let id: String = nanoid!(42);

        assert_eq!(id.len(), 42);
    }

    #[test]
    fn custom() {
        let id: String = nanoid!(42, &alphabet::SAFE);

        assert_eq!(id.len(), 42);
    }

    #[test]
    fn complex() {
        let id: String = nanoid!(4, &alphabet::SAFE, rngs::default);

        assert_eq!(id.len(), 4);
    }

    #[test]
    fn closure() {
        let uuid = "8936ad0c-9443-4007-9430-e223c64d4629";

        let id1 = nanoid!(20, &alphabet::SAFE, |_| uuid.as_bytes().to_vec());
        let id2 = nanoid!(20, &alphabet::SAFE, |_| uuid.as_bytes().to_vec());

        assert_eq!(id1, id2);
    }

    #[test]
    fn simple_expression() {
        let id: String = nanoid!(42 / 2);

        assert_eq!(id.len(), 21);
    }

    #[test]
    fn fnmut_closure() {
        // Test FnMut closures with mutable state
        let mut counter = 0u8;

        let id = nanoid!(10, &alphabet::SAFE, |size| {
            let mut bytes = vec![0u8; size];
            for byte in &mut bytes {
                *byte = counter;
                counter = counter.wrapping_add(1);
            }
            bytes
        });

        assert_eq!(id.len(), 10);
        assert!(counter > 0); // Verify the closure actually mutated the counter
    }
}

#[cfg(doctest)]
doc_comment::doctest!("../README.md");
