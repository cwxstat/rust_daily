fn main() {
    let strings = vec![
        String::from("1 2 3 4"),
        String::from("hello world"),
        String::from("goodbye world"),
        String::from("hello bobcat"),
        String::from("goodbye bobcat"),
        String::from("hello alabama"),
        String::from("goodbye alabama"),
    ];

    let filtered_strings: Vec<String> = strings
        .iter()
        .inspect(|s| println!("Before string: {:?}", s))
        .flat_map(|s| s.split_whitespace())
        .inspect(|s| println!("After string: {:?}", s))
        .filter(|s| s.contains("bob"))
        .map(|s| s.to_owned())
        .collect();

    println!("Filtered strings: {:?}", filtered_strings);
}