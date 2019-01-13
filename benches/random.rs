#![feature(test)]

extern crate nanoid;
extern crate test;

use test::Bencher;

#[bench]
fn os(b: &mut Bencher) {
    b.iter(|| nanoid::random::os(100));
}

#[bench]
fn standard(b: &mut Bencher) {
    b.iter(|| nanoid::random::standard(100));
}
