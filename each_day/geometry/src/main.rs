use std::fmt::Debug;
use std::f64::consts::PI;

// Define a Geometry trait with area and perim methods
pub trait Geometry {
    fn area(&self) -> f64;
    fn perim(&self) -> f64;
}

// Define a Rectangle struct
pub struct Rect {
    width: f64,
    height: f64,
}

// Implement the Geometry trait for Rect
impl Geometry for Rect {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perim(&self) -> f64 {
        2.0 * self.width + 2.0 * self.height
    }
}

// Define a Circle struct
pub struct Circle {
    radius: f64,
}

// Implement the Geometry trait for Circle
impl Geometry for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn perim(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

// Implement the Debug trait for Rect and Circle
impl Debug for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rect {{ width: {}, height: {} }}", self.width, self.height)
    }
}

impl Debug for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle {{ radius: {} }}", self.radius)
    }
}

// A generic measure function that works with any type implementing the Geometry trait
fn measure<T: Geometry + Debug>(g: &T) {
    println!("{:?}", g);
    println!("Area: {}", g.area());
    println!("Perimeter: {}", g.perim());
}

fn main() {
    let r = Rect { width: 3.0, height: 4.0 };
    let c = Circle { radius: 5.0 };

    measure(&r);
    measure(&c);
}
