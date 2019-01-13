#![feature(test)]

extern crate nanoid;
extern crate test;

use test::Bencher;

#[bench]
fn fast(b: &mut Bencher) {
    b.iter(|| nanoid::custom(10, &['a', 'b', 'c', 'd']));
}

#[bench]
fn universal(b: &mut Bencher) {
    b.iter(|| nanoid::custom(10, &['a', 'b', 'c', 'd', 'e']));
}
