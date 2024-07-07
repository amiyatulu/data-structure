use std::collections::BTreeMap;

fn main() {
    // creation of empty HashMap
    let mut book_reviews = BTreeMap::new();

    // Insert key and value
    book_reviews.insert("name".to_string(), "Tanushree".to_string());
    book_reviews.insert("title".to_string(), "Behera".to_string());
    println!("{:?}", book_reviews);
    let value1 = book_reviews.get(&"name".to_string()).unwrap();
    println!("{:?}", value1);

    if let Some(value) = book_reviews.get_mut(&"namex".to_string()) {
        *value = "Tantushree".to_string();
    }
    println!("{:?}", book_reviews);
}
