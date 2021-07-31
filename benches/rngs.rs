#![feature(test)]

extern crate test;

use test::Bencher;

#[bench]
fn default(b: &mut Bencher) {
    b.iter(|| nanoid::rngs::random_bytes(21));
}

#[bench]
fn nanoid_21(b: &mut Bencher) {
    b.iter(|| nanoid::nanoid!());
}

#[bench]
fn uuid_v4(b: &mut Bencher) {
    b.iter(|| uuid::Uuid::new_v4());
}
