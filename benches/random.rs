#![feature(test)]

extern crate test;
extern crate nanoid;

use test::Bencher;

#[bench]
fn os(b: &mut Bencher) {
    b.iter(|| nanoid::random::os(100));
}

#[bench]
fn standart(b: &mut Bencher) {
    b.iter(|| nanoid::random::standart(100));
}
