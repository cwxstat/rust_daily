# Chapter 1

This book was created with [mdBook](https://github.com/rust-lang/mdBook)

It's very simple to create a book with rust, with
interactive code snippets.

Everything here is created in the folder `/theBook`

### Building the book
Just run the following command:

```bash
./build.sh
```

### To Work on it Interactive

```bash
mdbook watch -o
```

Here's what's happening:

```bash
mdbook build
rm -rf ../docs
mv book ../docs
```

And, that's it!  Here is the first example of
interactive code.

```rust,editable
fn main() {
    // Some code
    println!("Hello, world!");
}
```

### A more advanced example

```rust,editable
macro_rules! test {
    // Arguments don't need to be separated by a comma.
    // Any template can be used!
    ($left:expr; and $right:expr) => {
        println!("{:?} and {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left && $right)
    };
    // ^ each arm must end with a semicolon.
    ($left:expr; or $right:expr) => {
        println!("{:?} or {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left || $right)
    };
}

fn main() {
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
}
```

## Useful Basic Guide

Sample with tests.

```rust,editable
fn add_integer(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("{}", add_integer(1, 2));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_integer() {
        assert_eq!(super::add_integer(1, 2), 3);
    }
}


```

## Sample with fmt skips

```rust,editable
fn vec_vec() -> Vec<Vec<i32>> {
    #[rustfmt::skip]
    let  vec = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];
    vec
}

fn main() {
    let vec = vec_vec();
    println!("{:?}", vec);
}
```