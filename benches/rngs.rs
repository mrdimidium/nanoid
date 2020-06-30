#![feature(test)]

extern crate test;

use test::Bencher;

#[bench]
fn default(b: &mut Bencher) {
    b.iter(|| nanoid::rngs::default(21));
}

#[bench]
fn non_secure(b: &mut Bencher) {
    b.iter(|| nanoid::rngs::non_secure(21));
}
