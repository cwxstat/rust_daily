// Create a trait for flying animals
pub trait Flyable {
    fn fly(&self);
}

// Create a trait for walking animals
pub trait Walkable {
    fn walk(&self);
}

// Implement a struct for birds that can fly and walk
pub struct Bird;

impl Flyable for Bird {
    fn fly(&self) {
        println!("The bird can fly.");
    }
}

impl Walkable for Bird {
    fn walk(&self) {
        println!("The bird can walk.");
    }
}

// Implement a struct for snakes that can only walk (slither)
pub struct Snake;

impl Walkable for Snake {
    fn walk(&self) {
        println!("The snake can slither.");
    }
}

fn main() {
    let bird = Bird;
    let snake = Snake;

    bird.fly();
    bird.walk();
    snake.walk();
}
