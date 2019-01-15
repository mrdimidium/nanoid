//! A tiny, secure, URL-friendly, unique string ID generator
//!
//! **Safe.** It uses cryptographically strong random APIs
//! and guarantees a proper distribution of symbols.
//!
//! **Compact.** It uses a larger alphabet than UUID (`A-Za-z0-9_~`)
//! and has a similar number of unique IDs in just 21 symbols instead of 36.
//!
//! ```toml
//! [dependencies]
//! nanoid = "0.1.3"
//! ```
//!
//! ```rust
//! extern crate nanoid;
//! use nanoid::nanoid;
//!
//! fn main() {
//!    let id = nanoid!(); //=> "Uakgb_J5m9g~0JDMbcJqLJ"
//! }
//! ```
//!
//! ## Usage
//!
//! ### Simple
//!
//! The main module uses URL-friendly symbols (`A-Za-z0-9_~`) and returns an ID
//! with 21 characters.
//!
//! ```rust
//! extern crate nanoid;
//! use nanoid::nanoid;
//!
//! fn main() {
//!    let id = nanoid!(); //=> "Uakgb_J5m9g~0JDMbcJqLJ"
//! }
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
//! extern crate nanoid;
//! use nanoid::nanoid;
//!
//! fn main() {
//!    let id = nanoid!(10); //=> "IRFa~VaY2b"
//! }
//! ```
//!
//! ### Custom Alphabet or Length
//!
//! If you want to change the ID's alphabet or length
//! you can use the low-level `custom` module.
//!
//! ```rust
//! extern crate nanoid;
//! use nanoid::nanoid;
//!
//! fn main() {
//!     let alphabet: [char; 16] = [
//!         '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f'
//!     ];
//!
//!    let id = nanoid!(10, &alphabet); //=> "4f90d13a42"
//! }
//! ```
//!
//! Alphabet must contain 256 symbols or less.
//! Otherwise, the generator will not be secure.
//!
//! ### Custom Random Bytes Generator
//!
//! You can replace the default safe random generator using the `complex` module.
//! For instance, to use a seed-based generator.
//!
//! ```rust
//! extern crate nanoid;
//! use nanoid::nanoid;
//!
//! fn randomByte () -> u8 {
//!     0
//! }
//!
//! fn main() {
//!     fn random (size: usize) -> Vec<u8> {
//!         let mut bytes: Vec<u8> = vec![0; size];
//!
//!         for i in 0..size {
//!             bytes[i] = randomByte();
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
//! If you want to use the same URL-friendly symbols with `format`,
//! you can get the default alphabet from the `url` module:
//!
//! ```rust
//! extern crate nanoid;
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

#![doc(
    html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png",
    html_favicon_url = "https://www.rust-lang.org/favicon.ico",
    html_root_url = "https://docs.rs/nanoid"
)]

extern crate rand;

pub mod alphabet;
pub mod rngs;

pub fn format(random: fn(usize) -> Vec<u8>, alphabet: &[char], size: usize) -> String {
    assert!(
        alphabet.len() <= u8::max_value() as usize,
        "The alphabet cannot be longer than a `u8` (to comply with the `random` function)"
    );

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
}

#[macro_export]
macro_rules! nanoid {
    // simple
    () => {
        $crate::format($crate::rngs::default, &$crate::alphabet::SAFE, 21)
    };

    // generate
    ($size:tt) => {
        $crate::format($crate::rngs::default, &$crate::alphabet::SAFE, $size)
    };

    // custom
    ($size:tt, $alphabet:expr) => {
        $crate::format($crate::rngs::default, $alphabet, $size)
    };

    // complex
    ($size:tt, $alphabet:expr, $random:expr) => {
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
}
