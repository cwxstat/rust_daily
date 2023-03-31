use std::fmt;

fn get_display() -> impl fmt::Display {
    "Hello, world!"
}

fn main() {
    println!("{}", get_display());
}
