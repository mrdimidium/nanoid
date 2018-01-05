extern crate rand;

mod random;
mod generator;
mod alphabets;

pub const SAFE_ALPHABET: &[char] = &alphabets::SAFE;

pub fn complex(size: usize, alphabet: &[char], random: fn(usize) -> Vec<u32>) -> String {
    let x = alphabet.len();

    // if (x == 2^n)
    let generator = if (x != 0) && ((x & (x - 1)) == 0) { generator::fast } else { generator::universal };

    generator(random, alphabet, size)
}

pub fn custom(size: usize, alphabet: &[char]) -> String {
    complex(size, alphabet, random::standart)
}

pub fn generate(size: usize) -> String {
    custom(size, &alphabets::SAFE)
}

pub fn simple() -> String {
    generate(21)
}
