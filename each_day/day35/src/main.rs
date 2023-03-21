// Define a simple macro named `say_hello` that takes one argument, $name.
macro_rules! say_hello {
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

// Define a macro named `calculate_area` that takes two arguments, $width and $height.
macro_rules! calculate_area {
    ($width:expr, $height:expr) => {
        ($width as f32) * ($height as f32)
    };
}

fn main() {
    // Use the `say_hello` macro with a string argument.
    say_hello!("Alice");

    // Use the `calculate_area` macro with two integer arguments.
    let width = 10;
    let height = 20.2;
    let area = calculate_area!(width, height);
    println!("The area of the rectangle is {} square units.", area);
}
