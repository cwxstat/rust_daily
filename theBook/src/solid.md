# SOLID Principles

## Dependency Inversion Principle

### High-level modules should not depend on low-level modules. Both should depend on abstractions.

```rust,editable
// Define an abstraction (trait) for data storage
pub trait DataStorage {
    fn save_data(&self, data: &str) -> Result<(), String>;
}

// Implement a low-level module for file storage
pub struct FileStorage;

impl DataStorage for FileStorage {
    fn save_data(&self, data: &str) -> Result<(), String> {
        println!("Saving data to a file: {}", data);
        // Implement file saving logic here
        Ok(())
    }
}

// Implement another low-level module for cloud storage
pub struct CloudStorage;

impl DataStorage for CloudStorage {
    fn save_data(&self, data: &str) -> Result<(), String> {
        println!("Saving data to the cloud: {}", data);
        // Implement cloud saving logic here
        Ok(())
    }
}

// High-level module that depends on the abstraction (trait) rather than concrete implementations
pub struct DataManager<T: DataStorage> {
    storage: T,
}

impl<T: DataStorage> DataManager<T> {
    pub fn new(storage: T) -> Self {
        DataManager { storage }
    }

    pub fn save_data(&self, data: &str) -> Result<(), String> {
        self.storage.save_data(data)
    }
}

fn main() {
    // Use FileStorage with DataManager
    let file_storage = FileStorage;
    let data_manager_file = DataManager::new(file_storage);
    data_manager_file.save_data("Some data for file storage").unwrap();

    // Use CloudStorage with DataManager
    let cloud_storage = CloudStorage;
    let data_manager_cloud = DataManager::new(cloud_storage);
    data_manager_cloud.save_data("Some data for cloud storage").unwrap();
}


```

## Interface Segregation Principle

### Many client-specific interfaces are better than one general-purpose interface.

```rust,editable
// Define smaller, more specific traits
trait Swimmer {
    fn swim(&self);
}

trait Flyer {
    fn fly(&self);
}

trait Walker {
    fn walk(&self);
}

// Define the structs that will implement the traits
struct Penguin;
struct Duck;
struct Eagle;

// Implement the traits for each struct
impl Swimmer for Penguin {
    fn swim(&self) {
        println!("Penguin swims.");
    }
}

impl Swimmer for Duck {
    fn swim(&self) {
        println!("Duck swims.");
    }
}

impl Flyer for Duck {
    fn fly(&self) {
        println!("Duck flies.");
    }
}

impl Flyer for Eagle {
    fn fly(&self) {
        println!("Eagle flies.");
    }
}

impl Walker for Eagle {
    fn walk(&self) {
        println!("Eagle walks.");
    }
}

// Use the specific traits in functions that require them
fn swim(swimmer: &impl Swimmer) {
    swimmer.swim();
}

fn fly(flyer: &impl Flyer) {
    flyer.fly();
}

fn walk(walker: &impl Walker) {
    walker.walk();
}

// Main function
fn main() {
    let penguin = Penguin;
    let duck = Duck;
    let eagle = Eagle;

    swim(&penguin);
    swim(&duck);
    fly(&duck);
    fly(&eagle);
    walk(&eagle);
}
```
