pub fn random_bytes(size: usize) -> Vec<u8> {
    let mut result: Vec<u8> = vec![0; size];

    getrandom::getrandom(&mut result[..])
        .unwrap_or_else(|err| panic!("could not retreive random bytes: {}", err));

    result
}

#[cfg(test)]
mod test_random_bytes {
    use super::*;

    #[test]
    fn generates_random_vectors() {
        let bytes = random_bytes(5);

        assert_eq!(bytes.len(), 5);
    }
}
