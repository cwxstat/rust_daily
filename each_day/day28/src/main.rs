fn modify_names(names: &mut Vec<&str>) {
    names.iter_mut().for_each(|i| {
        let new_name = match *i {
            "Bob" => format!("{}!, What's happening?", *i),
            _ => format!("Hi {}", *i),
        };
        *i = Box::leak(new_name.into_boxed_str());
    });
}

fn main() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    modify_names(&mut names);

    println!("{:?}", names);
}
