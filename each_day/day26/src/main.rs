fn greet<'a>(
    names: &mut Vec<&'a str>,
    target_name: &'a str,
    custom_message: &'a str,
    standard_message: &'a str,
) {
    for name in names.iter_mut() {
        *name = match name {
            &mut n if n == target_name => custom_message,
            _ => standard_message,
        }
    }
}

fn main() {
    let greeting = "Well, hello Ferris!";
    let mut names = vec!["Bob", "Frank", "Ferris"];
    greet(&mut names, "Ferris", greeting, "hi");
    println!("{:?}", names);
}

// Ref:
// https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/lifetimes.html
