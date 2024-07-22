# Handling Json

Sure! The `json!` macro in Rust is part of the `serde_json` crate, which provides a way to construct JSON values. Here's a simple example to illustrate how to parse JSON using the `json!` macro.

First, add the `serde` and `serde_json` crates to your `Cargo.toml` file:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

Now, here's an example of how you can parse a JSON object using the `json!` macro:

```rust
use serde_json::json;
use serde_json::Value;

fn main() {
    // Create a JSON object using the json! macro
    let data = json!({
        "name": "John Doe",
        "age": 30,
        "is_student": false,
        "address": {
            "street": "123 Main St",
            "city": "Anytown"
        },
        "phone_numbers": [
            "123-456-7890",
            "987-654-3210"
        ]
    });

    // Accessing data
    if let Some(name) = data.get("name") {
        println!("Name: {}", name);
    }

    if let Some(age) = data.get("age") {
        println!("Age: {}", age);
    }

    if let Some(is_student) = data.get("is_student") {
        println!("Is Student: {}", is_student);
    }

    if let Some(address) = data.get("address") {
        if let Some(street) = address.get("street") {
            println!("Street: {}", street);
        }

        if let Some(city) = address.get("city") {
            println!("City: {}", city);
        }
    }

    if let Some(phone_numbers) = data.get("phone_numbers") {
        if let Some(numbers) = phone_numbers.as_array() {
            for (i, number) in numbers.iter().enumerate() {
                println!("Phone Number {}: {}", i + 1, number);
            }
        }
    }
}
```

In this example:
1. The `json!` macro is used to create a JSON object with various fields.
2. The fields are accessed using the `.get()` method.
3. The `as_array()` method is used to iterate over the array of phone numbers.

You can run this program using `cargo run`, and it will output the parsed values from the JSON object.


## Load from json file

 Sure! Loading a JSON file in Rust using the `serde_json` crate involves reading the file and then parsing its contents. Here is an example of how you can do this:

1. First, ensure you have added the `serde` and `serde_json` crates to your `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

2. Next, create a JSON file named `data.json` with the following content:

```json
{
    "name": "John Doe",
    "age": 30,
    "is_student": false,
    "address": {
        "street": "123 Main St",
        "city": "Anytown"
    },
    "phone_numbers": [
        "123-456-7890",
        "987-654-3210"
    ]
}
```

3. Now, create a Rust program to load and parse this JSON file:

```rust
use serde_json::Value;
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    // Open the file
    let mut file = File::open("data.json")?;

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the string contents as JSON
    let data: Value = serde_json::from_str(&contents)?;

    // Accessing data
    if let Some(name) = data.get("name") {
        println!("Name: {}", name);
    }

    if let Some(age) = data.get("age") {
        println!("Age: {}", age);
    }

    if let Some(is_student) = data.get("is_student") {
        println!("Is Student: {}", is_student);
    }

    if let Some(address) = data.get("address") {
        if let Some(street) = address.get("street") {
            println!("Street: {}", street);
        }

        if let Some(city) = address.get("city") {
            println!("City: {}", city);
        }
    }

    if let Some(phone_numbers) = data.get("phone_numbers") {
        if let Some(numbers) = phone_numbers.as_array() {
            for (i, number) in numbers.iter().enumerate() {
                println!("Phone Number {}: {}", i + 1, number);
            }
        }
    }

    Ok(())
}
```

In this example:

1. The file `data.json` is opened and read into a string.
2. The string is parsed into a `serde_json::Value` using `serde_json::from_str`.
3. The data is accessed and printed similarly to the previous example.

Run this program using `cargo run`, and it will output the parsed values from the JSON file.
