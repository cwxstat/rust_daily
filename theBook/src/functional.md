# map,filter_map, collect


```rust,editable
fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    let r = names
        .iter()
        .filter_map(|name| match name.starts_with("F") {
            true => Some(name),
            false => None,
        })
        .map(|name| name.to_uppercase())
        .collect::<Vec<String>>();

    println!("{:?}", r);
}


```

Here's a more complicated example.  This is a common pattern in Rust.

```rust,editable
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
```