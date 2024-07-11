use sha1::{Sha1, Digest};

fn hash_password(password: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

fn verify_password(password: &str, hashed_password: &str) -> bool {
    let hashed_input = hash_password(password);
    hashed_input == hashed_password
}

fn main() {
    let password = "my_secure_password";
    let hashed_password = hash_password(password);
    // hased password: b02c6bf2b2b576d253f5c9db76999eebc4f00ad2

    println!("Original password: {}", password);
    println!("Hashed password: {}", hashed_password);

    let password_to_verify = "my_secure_password";
    let is_valid = verify_password(password_to_verify, &hashed_password);

    println!("Password verification result: {}", is_valid);
}
