use serde_json::json;
use serde_json::Value;
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = File::open("data.json")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("{}", contents);

    let data: Value = serde_json::from_str(&contents)?;

    println!("{}", data["name"]);

    println!("{}", data["age"]);

    println!("{}", data["address"]);

    println!("{}", data["address"]["city"]);

    println!("{}", data["phone_numbers"]);

    println!("{}", data["phone_numbers"][0]);

    Ok(())
}
