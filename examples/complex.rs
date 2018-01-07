extern crate rand;
extern crate nanoid;

use rand::Rng;

fn random (size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut bytes: Vec<u8> = vec![0; size];

    for i in 0..size {
        bytes[i] = rng.gen::<u8>();
    }

    bytes
}

fn main() {
    nanoid::complex(10, &['a', 'b', 'c', 'd', 'e', 'f'], random); //=> "fbaefaadeb"
}
