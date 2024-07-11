# Random number from seed

In Rust, you can generate random numbers using the `rand` crate. A seed is an initial value used by a pseudorandom number generator (PRNG) to produce a sequence of random numbers. Using the same seed will produce the same sequence of numbers, which is useful for reproducibility in testing and debugging.

Here's an example of generating random numbers from a seed in Rust:

1. Add the `rand` crate to your `Cargo.toml`:

```toml
[dependencies]
rand = "0.8"
```

2. Write your Rust code:

```rust
use rand::{SeedableRng, Rng};
use rand::rngs::StdRng;

fn main() {
    // Define a seed value
    let seed: [u8; 32] = [0; 32]; // You can use any array of 32 bytes as a seed

    // Create a seeded random number generator
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    // Generate some random numbers
    let random_number1: u32 = rng.gen();
    let random_number2: u32 = rng.gen();

    println!("Random number 1: {}", random_number1);
    println!("Random number 2: {}", random_number2);
}
```

### Explanation:

1. **Add `rand` crate**: The `rand` crate is a popular library in Rust for random number generation. Adding it to your `Cargo.toml` allows you to use its features.

2. **Seed Value**: A seed is a fixed-size array (in this case, 32 bytes) used to initialize the random number generator. The seed ensures that the sequence of random numbers generated will be the same each time you run the program with the same seed.

3. **SeedableRng and StdRng**: `SeedableRng` is a trait that allows a random number generator to be initialized with a seed. `StdRng` is a specific PRNG provided by the `rand` crate that implements `SeedableRng`.

4. **Generate Random Numbers**: Using the `gen` method from the `Rng` trait, you can generate random numbers. Since the PRNG is initialized with the same seed, it will produce the same sequence of random numbers every time the program is run.

By running this program, you'll always get the same random numbers (e.g., `Random number 1: 858204165` and `Random number 2: 3640784356`) as long as the seed remains the same.