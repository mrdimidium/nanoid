extern crate rand;

use rand::Rng;

const URL_SYMBOLS: [char; 64] = [
    '_', '~', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
];

fn random(size: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();

    let mut result: Vec<u32> = vec![0; size];

    for i in 0..size {
        result[i] = rng.gen::<u32>();
    }

    result
}

#[test]
fn test_random() {
    let bytes : Vec<u32> = random(5);

    assert_eq!(bytes.len(), 5);
}

pub extern fn simple(size: usize) -> String {
    let mut id = String::new();

    let bytes = random(size);

    for i in 0..size {
        let index = bytes[i] & 63;

        id.push(URL_SYMBOLS[index as usize]);
    }

    id
}
