fn add_integer(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("{}", add_integer(1, 2));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_integer() {
        assert_eq!(super::add_integer(1, 2), 3);
    }
}
