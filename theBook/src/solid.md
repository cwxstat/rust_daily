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
## DIP Another example

```rust,editable
pub trait Bird {
    fn bird_call(&self);
}

pub struct Chicken {
    pub sound: String,
}

impl Bird for Chicken {
    fn bird_call(&self) {
        println!("{}", self.sound);
    }
}

pub struct Duck {
    pub sound: String,
}

impl Bird for Duck {
    fn bird_call(&self) {
        println!("{}", self.sound);
    }
}

pub struct BirdCage<T: Bird> {
    pub bird: T,
}

impl<T: Bird> BirdCage<T> {
    pub fn new(bird: T) -> Self {
        Self { bird }
    }

    pub fn make_sound(&self) {
        self.bird.bird_call();
    }
}

fn main() {
    let chicken = Chicken {
        sound: String::from("Cluck!"),
    };
    let duck = Duck {
        sound: String::from("Quack!"),
    };

    let chicken_cage = BirdCage::new(chicken);
    let duck_cage = BirdCage::new(duck);

    chicken_cage.make_sound();
    duck_cage.make_sound();
}
```
## DIP bank example

```rust,editable
pub trait Transaction {
    fn execute(&self, balance: &mut f64);
}

pub struct Deposit {
    pub amount: f64,
}

impl Transaction for Deposit {
    fn execute(&self, balance: &mut f64) {
        *balance += self.amount;
        println!("Deposit: ${}", self.amount);
    }
}

pub struct Withdraw {
    pub amount: f64,
}

impl Transaction for Withdraw {
    fn execute(&self, balance: &mut f64) {
        if *balance >= self.amount {
            *balance -= self.amount;
            println!("Withdraw: ${}", self.amount);
        } else {
            println!("Withdrawal failed: Insufficient funds");
        }
    }
}

pub struct Bank {
    pub transactions: Vec<Box<dyn Transaction>>,
    pub balance: f64,
}

impl Bank {
    pub fn new() -> Self {
        Self {
            transactions: Vec::new(),
            balance: 0.0,
        }
    }

    pub fn add_transaction<T: Transaction + 'static>(&mut self, transaction: T) {
        self.transactions.push(Box::new(transaction));
    }

    pub fn process_transactions(&mut self) {
        for transaction in &self.transactions {
            transaction.execute(&mut self.balance);
        }
    }
}

fn main() {
    let deposit = Deposit { amount: 1000.0 };
    let withdraw = Withdraw { amount: 500.0 };
    let invalid_withdraw = Withdraw { amount: 2000.0 };

    let mut bank = Bank::new();
    bank.add_transaction(deposit);
    bank.add_transaction(withdraw);
    bank.add_transaction(invalid_withdraw);

    bank.process_transactions();
}

```
<blockquote>
In this example, we have a Transaction trait that represents the abstraction, and two structs, Deposit and Withdraw, that implement the Transaction trait. The Bank struct is a high-level module that depends on the Transaction trait, an abstraction, rather than depending on the concrete implementations (Deposit and Withdraw).
</blockquote>

<blockquote>
The Bank struct maintains a vector of Box<dyn Transaction> to store transactions. This allows it to work with any kind of transaction without having to know the specifics of each type of transaction. The main function demonstrates the usage of the Bank struct with both a Deposit and a Withdraw instance.
</blockquote>

<blockquote>
The above example follows the Dependency Inversion Principle, as both high-level and low-level modules depend on an abstraction, and the abstraction does not depend on any implementation details.
</blockquote>


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

# Open/Closed Principle
## Software entities should be open for extension, but closed for modification.

```rust,editable
// Define a trait `Shape` with a method `area`
pub trait Shape {
    fn area(&self) -> f64;
}

// Implement the `Shape` trait for a `Rectangle` struct
pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// Implement the `Shape` trait for a `Circle` struct
pub struct Circle {
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// A function that calculates the total area of a list of shapes
pub fn total_area(shapes: &[&dyn Shape]) -> f64 {
    shapes.iter().map(|shape| shape.area()).sum()
}

fn main() {
    let rectangle = Rectangle::new(5.0, 4.0);
    let circle = Circle::new(3.0);

    let shapes: Vec<&dyn Shape> = vec![&rectangle, &circle];

    println!("Total area: {}", total_area(&shapes));
}
```

# Liskov Substitution Principle
## Subtypes should be substitutable for their base types.

```rust,editable
pub trait Bird {
    fn fly(&self);
}

pub struct Swallow {
    pub name: String,
}

impl Swallow {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Bird for Swallow {
    fn fly(&self) {
        println!("{} the Swallow is flying.", self.name);
    }
}

pub struct Penguin {
    pub name: String,
}

impl Penguin {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Bird for Penguin {
    fn fly(&self) {
        println!("{} the Penguin cannot fly.", self.name);
    }
}

pub fn let_birds_fly(birds: &[&dyn Bird]) {
    for bird in birds {
        bird.fly();
    }
}

fn main() {
    let swallow = Swallow::new(String::from("Jack"));
    let penguin = Penguin::new(String::from("Penny"));

    let birds: Vec<&dyn Bird> = vec![&swallow, &penguin];

    let_birds_fly(&birds);
}
```


