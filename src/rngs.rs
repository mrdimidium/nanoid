use rand::{
    rngs::{SmallRng, StdRng},
    thread_rng, Rng, SeedableRng,
};

pub fn default(size: usize) -> Vec<u8> {
    let mut rng = StdRng::from_entropy();
    let mut result: Vec<u8> = vec![0; size];

    rng.fill(&mut result[..]);

    result
}

#[cfg(test)]
mod test_default {
    use super::*;

    #[test]
    fn generates_random_vectors() {
        let bytes = default(5);

        assert_eq!(bytes.len(), 5);
    }
}

pub fn non_secure(size: usize) -> Vec<u8> {
    let mut rng = SmallRng::from_entropy();
    let mut result = vec![0u8; size];

    rng.fill(&mut result[..]);

    result
}

#[cfg(test)]
mod test_non_secure {
    use super::non_secure;

    #[test]
    fn generates_random_vectors() {
        let bytes = non_secure(5);

        assert_eq!(bytes.len(), 5);
    }
}

/// Use the thread local Rng
pub fn thread_local(size: usize) -> Vec<u8> {
    let mut rng = thread_rng();
    let mut result = vec![0u8; size];

    rng.fill(&mut result[..]);

    result
}

#[cfg(test)]
mod test_thread_local {
    use super::thread_local;

    #[test]
    fn generates_random_vectors() {
        let bytes = thread_local(5);

        assert_eq!(bytes.len(), 5);
    }
}
