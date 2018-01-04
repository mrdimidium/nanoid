extern crate rand;

use rand::Rng;

pub fn gen(size: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();

    let mut result: Vec<u32> = vec![0; size];

    for i in 0..size {
        result[i] = rng.gen::<u32>();
    }

    result
}

#[test]
fn generates_random_vectors() {
    let bytes : Vec<u32> = gen(5);

    assert_eq!(bytes.len(), 5);
}
