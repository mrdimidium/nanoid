extern crate nanoid;
extern crate rand;

use rand::Rng;

fn random(size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    rng.gen_iter::<u8>().take(size).collect()
}

fn main() {
    nanoid::complex(10, &['a', 'b', 'c', 'd', 'e', 'f'], random); //=> "fbaefaadeb"
}
