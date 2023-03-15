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

# Lifetimes

This gets a little more complicated.  But, it's important.

We have to make sure all the variables have the same lifetime.

```rust,editable
fn greet<'a>(
    names: &mut Vec<&'a str>,
    target_name: &'a str,
    custom_message: &'a str,
    standard_message: &'a str,
) {
    for name in names.iter_mut() {
        *name = match name {
            &mut n if n == target_name => custom_message,
            _ => standard_message,
        }
    }
}

fn main() {
    let greeting = "Well, hello Ferris!";
    let mut names = vec!["Bob", "Frank", "Ferris"];
    greet(&mut names, "Ferris", greeting, "hi");
    println!("{:?}", names);
}

// Ref:
// https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/lifetimes.html
```

# Working with Files

```rust
use std::fs::File;
use std::io::prelude::*;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn modify_names(names: &mut Vec<&str>, custom_message: &str) {
    names.iter_mut().for_each(|i| {
        let new_name = match *i {
            "Bob" => format!("{}!, What's happening?", *i),
            _ => format!("Hi {} {}", *i, custom_message),
        };
        *i = Box::leak(new_name.into_boxed_str());
    });
}

fn main() {
    match read_file("./data/file.txt") {
        Ok(contents) => {
            let mut lines = contents.lines().collect();
            modify_names(&mut lines, ", custom message!");
            println!("{:?}", lines);
        }
        Err(e) => panic!("Error reading file: {:?}", e),
    }
}
```

# Working with Errors

```rust,editable
use std::io::{Error, ErrorKind};

fn find(substring: &str) -> Result<String, Error> {
    let strings: Vec<String> = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
    ];

    if let Some(s) = strings.iter().find(|s| s.contains(substring)) {
        println!("Found '{}' in '{}'", substring, s);
        Ok(s.to_string())
    } else {
        Err(Error::new(
            ErrorKind::NotFound,
            format!("No match for '{}'", substring),
        ))
    }
}

fn main() {
    let result = find("an");

    if result.is_err() {
        println!("Error: {:?}", result.err());
    } else {
        println!("Result: {:?}", result.unwrap());
    }
}


```

## Favorite 

```rust,editable
fn main() {
    let strings = vec![
        String::from("hello"),
        String::from("world"),
        String::from("bobcat"),
        String::from("bob"),
        String::from("alabama"),
    ];

    let mut filtered_strings: Vec<String> = strings
        .iter()
        .filter(|s| s.contains("bob"))
        .inspect(|s| println!("Filtered string: {:?}", s))
        .map(|s| s.clone())
        .collect();

    filtered_strings.sort();
    println!("Filtered strings: {:?}", filtered_strings);
}

```


## Here's another one

```rust,editable
fn main() {
    let strings = vec![
        String::from("hello"),
        String::from("world"),
        String::from("bobcat"),
        String::from("bob"),
        String::from("alabama"),
    ];

    let mut filtered_strings: Vec<String> = strings
        .iter()
        .filter_map(|s| {
            if s.contains("bob") {
                Some(s.clone())
            } else {
                None
            }
        })
        .collect();

    filtered_strings.sort();
    println!("Filtered strings: {:?}", filtered_strings);
}

```

## Useful

This can also be useful

```rust,editable
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    if let Some(third_number) = numbers.iter().nth(2) {
        println!("The third number is {}", third_number);
    } else {
        println!("There is no third number.");
    }
}


```

## There's more

```rust,editable
fn main() {
    let strings = vec![
        String::from("hello"),
        String::from("world"),
        String::from("bobcat"),
        String::from("bob"),
        String::from("alabama"),
    ];

    let mut filtered_strings: Vec<String> = strings
        .iter()
        .filter(|s| s.contains("bob"))
        .inspect(|s| println!("Filtered string: {:?}", s))
        .filter(|s| sporky(s))
        .map(|s| s.clone())
        .collect();

    filtered_strings.sort();
    println!("Filtered strings: {:?}", filtered_strings);
}

fn sporky(string: &str) -> bool {
    if string.contains("cat") {
        println!("{} is sporky", string);
        true
    } else {
        println!("{} is not sporky", string);
        false
    }
}

```

## You gotta love maps

    

```rust,editable
fn main() {
    let strings = vec![
        String::from("1 2 3 4"),
        String::from("hello world"),
        String::from("goodbye world"),
        String::from("hello bobcat"),
        String::from("goodbye bobcat"),
        String::from("hello alabama"),
        String::from("goodbye alabama"),
    ];

    let filtered_strings: Vec<String> = strings
        .iter()
        .inspect(|s| println!("Before string: {:?}", s))
        .flat_map(|s| s.split_whitespace())
        .inspect(|s| println!("After string: {:?}", s))
        .filter(|s| s.contains("bob"))
        .map(|s| s.to_owned())
        .collect();

    println!("Filtered strings: {:?}", filtered_strings);
}
```
