fn main() {
    println!("Hello, world!");

    let now = chrono::Local::now();

    println!("Current time: {}", now);

    let time = time::now_utc();

    println!("Current UTC time: {:?}", time);

    let json_str = r#"{"key": "value"}"#;

    let json_value: serde_json::Value = serde_json::from_str(json_str).unwrap();

    println!("Parsed JSON: {}", json_value);

    let img = image::open("example.png").unwrap();
    println!("Opened image: {:?}", img.as_bytes());
}
