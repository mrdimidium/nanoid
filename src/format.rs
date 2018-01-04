pub fn format(random: fn(usize) -> Vec<u32>, alphabet: &[char], size: usize) -> String {
    let mask = (2 << ((alphabet.len() as f64 - 1.0).ln() / 2.0_f64.ln()) as i64) - 1;
    let step: usize = (1.6_f64 * (mask * size) as f64).ceil() as usize;

    let mut id = String::new();

    'main: loop {
        let bytes = random(step);

        for i in 0..step {
            let byte: usize = bytes[i] as usize & mask;

            if alphabet.len() > byte {
                id.push(alphabet[byte]);

                if id.len() == size {
                    break 'main;
                }
            }
        }
    }

    id
}

#[test]
fn generates_random_string() {
    fn random (size: usize) -> Vec<u32> {
        let sequence: Vec<u32> = vec![2, 255, 0, 1];

        let mut bytes: Vec<u32> = vec![];

        let mut i =  0;
        while i < size {
            let (elements, _) = sequence.split_at(if size - i > sequence.len() { sequence.len() } else { size - i });

            for &el in elements {
                bytes.push(el);
            }

            i += sequence.len();
        }

        bytes
    }

    assert_eq!(format(random, &['a', 'b', 'c'], 4), "cabc");
}
