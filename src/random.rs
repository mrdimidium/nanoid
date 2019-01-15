use rand::{rngs::OsRng, thread_rng, Rng};

pub fn standard(size: usize) -> Vec<u8> {
    let mut result: Vec<u8> = vec![0; size];

    thread_rng().fill(&mut result[..]);

    result
}

#[cfg(test)]
mod test_standard {
    use super::*;

    #[test]
    fn generates_random_vectors() {
        let bytes = standard(5);

        assert_eq!(bytes.len(), 5);
    }
}

pub fn os(size: usize) -> Vec<u8> {
    let mut rng = OsRng::new().unwrap();
    let mut result = vec![0u8; size];

    rng.fill(&mut result[..]);

    result
}

#[cfg(test)]
mod test_secure {
    use super::os;

    #[test]
    fn generates_random_vectors() {
        let bytes = os(5);

        assert_eq!(bytes.len(), 5);
    }
}
