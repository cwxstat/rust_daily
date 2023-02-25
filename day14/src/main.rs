mod my_module;
use my_module::{my_module2::submodule2, submodule1};
use crate::my_module::my_module2::{add_integer, get_integer_vector};


#[allow(dead_code)]
fn main() {
    let sum = submodule1::add(1, 2);
    let sum2 = submodule1::add2(1, 2);
    let sum3 = submodule2::add(1, 2);
    add_integer(10);
    add_integer(20);
    add_integer(30);

    println!("Sum: {}", sum);
    println!("Sum2: {}", sum2);
    println!("Sum3: {}", sum3);
    println!("result {}", get_integer_vector().lock().unwrap()[0]);
}
