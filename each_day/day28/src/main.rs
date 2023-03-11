use std::fs::File;
use std::io::prelude::*;

fn read_file(path: &str) -> String {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("Error opening file: {:?}", e),
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(e) => panic!("Error reading file: {:?}", e),
    }
    contents
}

fn modify_names(names: &mut Vec<&str>, custom_message: &str) {
    names.iter_mut().for_each(|i| {
        let new_name = match *i {
            "Bob" => format!("{}!, What's happening?", *i),
            _ => format!("Hi {} {}", *i, custom_message),
        };
        *i = Box::leak(new_name.into_boxed_str());
    });
}

fn main() {
    let input_string = read_file("./data/file.txt");

    let mut lines: Vec<&str> = input_string.lines().collect();

    modify_names(&mut lines, ", custom message!");

    println!("{:?}", lines);
}
