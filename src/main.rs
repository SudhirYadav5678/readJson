use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)] // Automatically derive serialization and deserialization traits
struct Paragraph {
    name: String,
    phone: String,
    class: String,
}

fn main() {
    println!("Hello, world!");

    let json = r#"
    {
        "name": "Sudhir",
        "phone": "123454",
        "class": "dddjdfj"
    }"#;

    let parsed = read_json_typed(json);

    // Print parsed object
    println!(
        "Parsed Paragraph: Name = {}, Phone = {}, Class = {}",
        parsed.name, parsed.phone, parsed.class
    );
}

fn read_json_typed(raw_json: &str) -> Paragraph {
    serde_json::from_str(raw_json).unwrap() // Parse JSON to Paragraph struct
}
