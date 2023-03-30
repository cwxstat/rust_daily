struct Cat {
    weight: f32,
    name: String,
    sound: String,
    speed: f32,
}

struct Dog {
    weight: f32,
    name: String,
    sound: String,
    speed: f32,
}

trait Animal {
    fn new(name: &str, weight: f32) -> Self;
    fn noise(&self) -> &str;
    fn run(&self) -> f32;
    fn weight(&self) -> f32;
}

impl Animal for Cat {
    fn new(name: &str, weight: f32) -> Self {
        Cat {
            weight,
            name: String::from(name),
            sound: String::from("Meow"),
            speed: 30.0,
        }
    }

    fn noise(&self) -> &str {
        &self.sound
    }

    fn run(&self) -> f32 {
        self.speed
    }
    fn weight(&self) -> f32 {
        self.weight
    }
}

impl Animal for Dog {
    fn new(name: &str, weight: f32) -> Self {
        Dog {
            weight,
            name: String::from(name),
            sound: String::from("Woof"),
            speed: 45.0,
        }
    }

    fn noise(&self) -> &str {
        &self.sound
    }

    fn run(&self) -> f32 {
        self.speed
    }

    fn weight(&self) -> f32 {
        self.weight
    }
}

fn main() {
    let cat: Cat = Animal::new("Fluffy", 4.5);
    let dog: Dog = Animal::new("Rover", 30.0);

    println!(
        "{} the cat says {} and runs at {} km/h.",
        cat.name,
        cat.noise(),
        cat.run()
    );
    println!(
        "{} the dog says {} and runs at {} km/h.",
        dog.name,
        dog.noise(),
        dog.run()
    );
}
