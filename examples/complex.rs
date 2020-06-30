use nanoid::nanoid;

use rand::distributions::Standard;
use rand::{thread_rng, Rng};

fn random(size: usize) -> Vec<u8> {
    let mut rng = thread_rng();

    rng.sample_iter(&Standard).take(size).collect()
}

fn main() {
    nanoid!(10, &['a', 'b', 'c', 'd', 'e', 'f'], random); //=> "fbaefaadeb"
}
