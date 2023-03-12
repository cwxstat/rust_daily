use std::fs::File;
use std::io::prelude::*;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
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
    match read_file("./data/file.txt") {
        Ok(contents) => {
            let mut lines = contents.lines().collect();
            modify_names(&mut lines, ", custom message!");
            println!("{:?}", lines);
        }
        Err(e) => panic!("Error reading file: {:?}", e),
    }
}
