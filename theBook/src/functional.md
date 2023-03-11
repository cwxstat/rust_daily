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