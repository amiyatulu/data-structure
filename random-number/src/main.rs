use rand::Rng;

fn main() {
    // Create a random number generator
    // Retrieve the lazily-initialized thread-local random number generator, seeded by the system.
    let mut rng = rand::thread_rng();

    // Generate a random number between 0 and 100
    let random_number: u32 = rng.gen_range(0..10);

    println!("Random number: {}", random_number);
}
