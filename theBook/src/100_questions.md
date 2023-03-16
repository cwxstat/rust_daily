# 100+ Questions

- What is Rust and what makes it unique?

- What are the advantages of using Rust over other languages?

- Explain the concepts of ownership, borrowing, and lifetimes in Rust.

- What is a variable shadowing in Rust?

- What is the difference between mutable and immutable variables?

- How is memory safety ensured in Rust?

- What is the borrow checker and how does it work?

- What are the differences between a slice and an array in Rust?
- How do you define a function in Rust?
- What is pattern matching and how is it used in Rust?
- What is a tuple and how does it differ from an array?
- What is the match expression and how does it work?
- What are enums and how do you use them?
- Explain the concept of structs in Rust.
- How do you implement methods for a struct?
- What are traits and how are they used in Rust?
- What are the differences between a trait and an interface in other languages?
- What is a module in Rust and how do you create one?
- How do you handle errors in Rust?
- Explain the difference between panic and Result.
- What is the Option type and when should you use it?
- What is an iterator in Rust and how do you create one?
- How do you work with strings in Rust?
- What is the difference between a String and a &str in Rust?
- How do you implement generics in Rust?
- What is a macro in Rust and how do you define one?
- Explain the differences between macros and functions.
- What is the standard library and what are some of its components?
- How does Rust handle concurrency?
- What is the difference between threads and tasks in Rust?
- Explain the concept of async/await in Rust.
- What is a closure and how does it work in Rust?
- What are the differences between a closure and a function?
- How do you work with files and directories in Rust?
- Explain the concept of a Result type and its use cases.
- How do you perform error propagation in Rust?
-- What is the difference between a Vec and an array in Rust?
-  What is a HashMap and how do you use it in Rust?
-  Explain the concept of reference counting in Rust.
-  What are the differences between Arc and Rc?
-  What is a smart pointer and how does it work in Rust?
-  Explain the concept of interior mutability in Rust.
-  What are the differences between Cell and RefCell?
-  How does Rust handle null values?
-  What is the difference between PartialEq and Eq?
-  What is a type alias and how do you create one?
-  Explain the concept of associated types in Rust.
-  What is an associated function and how do you define one?
-  How do you implement multiple traits for a single type?
-  What is the purpose of the drop function in Rust?
-  Explain the concept of the Deref trait.
-  What is the purpose of the PhantomData type?
-  What is the difference between Send and Sync?
-  Explain the concept of unsafe code in Rust.
-  What is a raw pointer and how do you use it?
-  How do you define a constant in Rust?
-  What is a static variable and how do you define one?
-  What is a lazy_static and when should you use it?
- How do you define and use custom error types in Rust?
-  What is the difference between Clone and Copy traits?
-  What is a lifetime and why is it important in Rust?
-  How do you specify lifetime annotations in Rust?
-  What is the 'static lifetime and when is it used?
-  What is the purpose of the Sized trait?
-  Explain the concept of type erasure in Rust.
-  How do you use cargo and what is its purpose?
-  How do you create and use a dependency in a Rust project?
-  What is a workspace in Cargo and how do you create one?
-  Explain the concept of conditional compilation in Rust.
-  What is a build script and how do you use it in a Rust project?
-  What is a feature flag and how do you use it in Rust?
-  Explain the concept of testing in Rust.
-  How do you write unit tests and integration tests in Rust?
-  What is the purpose of the #[cfg(test)] attribute?
-  How do you perform benchmarking in Rust?
-  What is the difference between a no_std and a std Rust project?
-  How do you compile Rust code to WebAssembly?
-  What is serde and how do you use it in Rust?
-  How do you implement serialization and deserialization in Rust?
-  Explain the concept of FFI (Foreign Function Interface) in Rust.
-  How do you call C code from Rust and vice versa?
-  What is the purpose of the Pin type in Rust?
-  Explain the concept of zero-cost abstractions in Rust.
-  What is the difference between a Box, a Rc, and an Arc?

<blockquote>
Box: A Box provides heap allocation for a single value. It is useful when you want to allocate data on the heap rather than the stack. In this example, we create a Box containing a Foo instance and print its value.
</blockquote>

<blockquote>
Rc (Reference Counting): Rc is used when you want to share ownership of an instance across multiple parts of your code. It tracks the number of references to an instance and deallocates the memory when the reference count goes down to zero. In this example, we create an Rc containing a Foo instance, clone it, and print the value of both the original Rc and the clone.
</blockquote>

<blockquote>
Arc (Atomic Reference Counting): Arc is similar to Rc, but it is thread-safe, allowing shared ownership across multiple threads. It uses atomic operations to manage the reference count, which makes it safe to use concurrently. In this example, we create an Arc containing a Foo instance, clone it, spawn a new thread that uses the cloned instance, and print the value of both the original Arc and the clone in the new thread.
</blockquote>

```rust,editable
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

struct Foo {
    value: i32,
}

fn main() {
    // Box
    let foo_box = Box::new(Foo { value: 42 });
    println!("Box value: {}", foo_box.value);

    // Rc (Reference Counting)
    let foo_rc = Rc::new(Foo { value: 42 });
    let foo_rc_clone = Rc::clone(&foo_rc);
    println!("Rc value: {}", foo_rc.value);
    println!("Rc clone value: {}", foo_rc_clone.value);

    // Arc (Atomic Reference Counting)
    let foo_arc = Arc::new(Foo { value: 42 });
    let foo_arc_clone = Arc::clone(&foo_arc);

    let handle = thread::spawn(move || {
        println!("Arc value in thread: {}", foo_arc_clone.value);
    });

    println!("Arc value: {}", foo_arc.value);

    handle.join().unwrap();
}


```


-  What is a Mutex and how do you use it in Rust?
-  What is a RwLock and how do you use it in Rust?
-  How do you implement a custom iterator in Rust?
-  What is the purpose of the From and Into traits?

```rust,editable
#[derive(Debug)]
struct Meter(u32);

#[derive(Debug)]
struct Feet(u32);

// Implementing the From trait for converting Feet into Meter
impl From<Feet> for Meter {
    fn from(feet: Feet) -> Self {
        Meter((feet.0 as f32 * 0.3048) as u32)
    }
}

// Implementing the Into trait for converting Meter into Feet
impl Into<Feet> for Meter {
    fn into(self) -> Feet {
        Feet((self.0 as f32 * 3.28084) as u32)
    }
}

fn main() {
    // Using the From trait
    let feet = Feet(100);
    let meter: Meter = Meter::from(feet);
    println!("100 feet is equal to {:?} meters", meter);

    // Using the Into trait
    let meter = Meter(30);
    let feet: Feet = meter.into();
    println!("30 meters is equal to {:?} feet", feet);
}

```

-  What is the difference between a ref and a ref mut in Rust?
-  How do you use conditional statements in Rust?
-  What are the different types of loops in Rust and how do you use them?
-  Explain the concept of a tuple struct in Rust.
-  How do you implement a recursive function in Rust?
-  What is the difference between a function and a method in Rust?
-  What is a constructor and how do you create one in Rust?
-  Explain the concept of type inference in Rust.
-  What is the purpose of the AsRef and AsMut traits?
-  What is a nightly build of Rust and when should you use it?
-  What are the different editions of Rust and how do they differ?
-  What resources do you recommend for learning Rust?




