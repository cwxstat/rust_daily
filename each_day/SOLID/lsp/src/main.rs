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
