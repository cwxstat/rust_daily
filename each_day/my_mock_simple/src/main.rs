#![allow(unused_imports)]

use my_mock_simple::{Greeter, RealGreeter};

fn main() {
    let greeter = RealGreeter;
    println!("{:?}", greeter.greet("Alice"));
    // "Hello, Alice!"
}
