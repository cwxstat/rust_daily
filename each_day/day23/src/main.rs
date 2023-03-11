fn main() {
    let now = std::time::SystemTime::now();
    let since_the_epoch = now
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");
    println!("Hello, world! {}", since_the_epoch.as_secs());
}
