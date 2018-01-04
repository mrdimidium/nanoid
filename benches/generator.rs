#![feature(test)]

extern crate test;
extern crate nanoid;

use test::Bencher;

#[bench]
fn fast(b: &mut Bencher) {
    b.iter(|| nanoid::custom(10, &['a', 'b', 'c', 'd']));
}

#[bench]
fn universal(b: &mut Bencher) {
    b.iter(|| nanoid::custom(10, &['a', 'b', 'c', 'd', 'e']));
}
