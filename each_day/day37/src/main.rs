// Define a macro named `fibonacci` that takes one argument: the name of the function to be generated
macro_rules! fibonacci {
    ($function_name:ident) => {
        // Define the generated function
        fn $function_name(n: u32) -> u64 {
            match n {
                0 => 0,
                1 => 1,
                _ => $function_name(n - 1) + $function_name(n - 2),
            }
        }
    };
}

// Use the `fibonacci` macro to generate a function named `fib`
fibonacci!(fib);

fn main() {
    let n = 10;
    println!("The {}th Fibonacci number is {}", n, fib(n));
}
