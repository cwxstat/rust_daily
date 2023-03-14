fn main() {
    let n = 10; // Change this to the number of Fibonacci numbers you want
    let mut prev = 0;
    let mut curr = 1;

    for _i in 0..n {
        print!("{} ", curr);

        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    println!();
}