extern crate rand;
extern crate nanoid;

use rand::Rng;

fn main() {
    fn random (size: usize) -> Vec<u32> {
        let mut rng = rand::thread_rng();
        let mut bytes: Vec<u32> = vec![0; size];

        for i in 0..size {
            bytes[i] = rng.gen::<u32>();
        }

        bytes
    }

    nanoid::complex(10, &['a', 'b', 'c', 'd', 'e', 'f'], random); //=> "fbaefaadeb"
}
