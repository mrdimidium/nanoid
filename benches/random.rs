#![feature(test)]

extern crate test;
extern crate nanoid;

use test::Bencher;

#[bench]
fn os(b: &mut Bencher) {
    b.iter(|| nanoid::random::os(100));
}

#[bench]
fn standard(b: &mut Bencher) {
    b.iter(|| nanoid::random::standard(100));
}
