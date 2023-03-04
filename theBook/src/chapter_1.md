# Chapter 1

This book was created with [mdBook](https://github.com/rust-lang/mdBook)

It's very simple to create a book with rust, with
interactive code snippets.

Everything here is created in the folder `/theBook`

```bash
mdbook build
rm -rf ../docs
mv book ../docs
```

```rust,editable
fn main() {
    // Some code
    println!("Hello, world!");
}
```

## Subchapter
