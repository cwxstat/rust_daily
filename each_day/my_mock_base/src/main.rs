#![allow(unused_imports)]

use my_mock_base::{Foo, LifeExpectancy};

fn years_left(f: ) -> u32 {
    86 - 32
}


fn main() {
    let life = Foo::new("Alice".to_string(), 32, "123".to_string());
    println!("{:?}", life.years_left());

}
