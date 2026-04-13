use nanoid::nanoid;
use rand::{rngs::StdRng, Rng, SeedableRng};

fn main() {
    // Example 1: Using a seeded RNG (stateful, requires FnMut)
    let mut rng = StdRng::seed_from_u64(42);

    let id1 = nanoid!(10, &nanoid::alphabet::SAFE, |size| {
        let mut bytes = vec![0u8; size];
        rng.fill(&mut bytes[..]);
        bytes
    });

    println!("Seeded ID 1: {}", id1);

    let id2 = nanoid!(10, &nanoid::alphabet::SAFE, |size| {
        let mut bytes = vec![0u8; size];
        rng.fill(&mut bytes[..]);
        bytes
    });

    println!("Seeded ID 2: {}", id2);

    // Example 2: Using a counter-based generator (requires FnMut)
    let mut counter = 0u8;

    let id3 = nanoid!(
        10,
        &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
        |size| {
            let mut bytes = vec![0u8; size];
            for byte in &mut bytes {
                *byte = counter;
                counter = counter.wrapping_add(17); // Use a prime for better distribution
            }
            bytes
        }
    );

    println!("Counter-based ID: {}", id3);

    // Example 3: Demonstrating reproducible IDs with same seed
    println!("\nReproducible IDs with same seed:");

    let seed = 12345u64;
    let alphabet = &['a', 'b', 'c', 'd', 'e', 'f'];

    let mut rng1 = StdRng::seed_from_u64(seed);
    let id_a = nanoid!(8, alphabet, |size| {
        let mut bytes = vec![0u8; size];
        rng1.fill(&mut bytes[..]);
        bytes
    });

    let mut rng2 = StdRng::seed_from_u64(seed);
    let id_b = nanoid!(8, alphabet, |size| {
        let mut bytes = vec![0u8; size];
        rng2.fill(&mut bytes[..]);
        bytes
    });

    println!("ID A: {}", id_a);
    println!("ID B: {}", id_b);
    println!("Are they equal? {}", id_a == id_b);
}
