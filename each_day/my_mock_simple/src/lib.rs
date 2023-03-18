#![allow(unused_imports)]
use mockall::automock;

// Implement the trait for a struct
pub struct RealGreeter;

impl Greeter for RealGreeter {
    fn greet(&self, name: &str) -> String {
        format!("Hello, {}!", name)
    }
}

// Create a mock version of the trait using the `automock` attribute
#[cfg_attr(test, automock)]
pub trait Greeter {
    fn greet(&self, name: &str) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    #[test]
    fn test_greeter_mock() {
        // Create a new mock greeter
        let mut greeter = MockGreeter::new();

        // Expect the `greet` method to be called with "Alice" and return "Hi, Alice!"
        greeter
            .expect_greet()
            .with(eq("Alice"))
            .returning(|name| format!("Hi, {}!", name));

        // Test that the mock greeter returns the expected output
        assert_eq!("Hi, Alice!", greeter.greet("Alice"));
    }
}
