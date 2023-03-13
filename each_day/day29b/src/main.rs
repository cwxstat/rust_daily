use std::io::{Error, ErrorKind};

fn find(substring: &str) -> Result<String, Error> {
    let strings: Vec<String> = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
    ];

    if let Some(s) = strings.iter().find(|s| s.contains(substring)) {
        println!("Found '{}' in '{}'", substring, s);
        Ok(s.to_string())
    } else {
        Err(Error::new(
            ErrorKind::NotFound,
            format!("No match for '{}'", substring),
        ))
    }
}

fn main() {
    let result = find("an");

    if result.is_err() {
        println!("Error: {:?}", result.err());
    } else {
        println!("Result: {:?}", result.unwrap());
    }
}
