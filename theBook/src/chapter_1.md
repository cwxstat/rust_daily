# Chapter 1

This book was created with [mdBook](https://github.com/rust-lang/mdBook)

It's very simple to create a book with rust, with
interactive code snippets.

Everything here is created in the folder `/theBook`

### Create a new book
Just run the following command:
```bash
./build.sh
```

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


