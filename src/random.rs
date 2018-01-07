use rand;

use rand::Rng;
use rand::os::OsRng;

pub fn standart(size: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();

    let mut result: Vec<u32> = vec![0; size];

    for i in 0..size {
        result[i] = rng.gen::<u32>();
    }

    result
}

#[cfg(test)]
mod test_standart {
    use super::*;

    #[test]
    fn generates_random_vectors() {
        let bytes : Vec<u32> = standart(5);

        assert_eq!(bytes.len(), 5);
    }
}

pub fn os(size: usize) -> Vec<u32> {
    let mut random = OsRng::new().unwrap();
    let mut result: Vec<u32> = vec![0; size];

    for i in 0..size {
        result[i] = random.next_u32();
    }

    result
}

mod test_secure {
    use super::*;

    #[test]
    fn generates_random_vectors() {
        let bytes: Vec<u32> = os(5);

        assert_eq!(bytes.len(), 5);
    }
}
