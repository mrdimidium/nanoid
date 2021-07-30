pub fn default(size: usize) -> Vec<u8> {
    let mut result: Vec<u8> = vec![0; size];

    getrandom::getrandom(&mut result[..])
        .unwrap_or_else(|err| panic!("could not retreive random bytes: {}", err));

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
    let mut result = vec![0u8; size];

    getrandom::getrandom(&mut result[..])
        .unwrap_or_else(|err| panic!("could not retreive random bytes: {}", err));

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
