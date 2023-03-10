fn greet<'a>(names: &mut Vec<&'a str>, target_name: &'a str, custom_message: &'a str) {
    for name in names.iter_mut() {
        *name = match name {
            &mut n if n == target_name => custom_message,
            _ => "Hello",
        }
    }
}

fn main() {
    let greeting = "well hello Ferris!";
    let mut names = vec!["Bob", "Frank", "Ferris"];
    greet(&mut names, "Ferris", greeting);
    println!("{:?}", names);
}
