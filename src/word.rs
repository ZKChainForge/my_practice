use std::collections::HashMap;

pub fn run() {
    let text = "rust is fast and rust is memory safe";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    for (word, count) in &map {
        println!("{}: {}", word, count);
    }
}