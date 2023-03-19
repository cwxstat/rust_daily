#![allow(unused_imports)]
#![allow(dead_code)]
use mockall::automock;

// Implement the trait for a struct
pub struct Foo {
    pub name: String,
    pub age: u32,
    pub badge: String,

}

#[automock]
impl Foo {
    pub fn new(name: String, age: u32, badge: String) -> Foo {
        Foo {
            name,
            age,
            badge,
        }
    }

}

impl LifeExpectancy for Foo {
    fn years_left(&self) -> u32 {
        86 - self.age
    }
}

#[cfg_attr(test, automock)]
pub trait LifeExpectancy
{
    fn years_left(&self) -> u32;
}


#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    #[test]
    fn test_foo_mock() {
        // Create a new mock foo
        //let mut _foo = MockFoo::new("Alice".to_string(), 32, "123".to_string());
        let mut life = MockLifeExpectancy::new();

        life
            .expect_years_left()
            .returning(|| 86 - 32);

        assert_eq!(54, life.years_left());
    }
}

