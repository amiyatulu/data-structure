use rand::{SeedableRng, Rng};
use rand::rngs::StdRng;

fn main() {
    // Define a seed value
    let seed: [u8; 32] = [1; 32]; // You can use any array of 32 bytes as a seed

    // Create a seeded random number generator
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    // Generate some random numbers
    let random_number1: u32 = rng.gen_range(0..10);
    let random_number2: u32 = rng.gen_range(0..10);
    let random_number3: u32 = rng.gen_range(0..10);

    println!("Random number 1: {}", random_number1);
    println!("Random number 2: {}", random_number2);
    println!("Random number 3: {}", random_number3);

}