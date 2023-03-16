fn main() {
    println!("Hello, world!");
}

fn add_two(a: i32) -> i32 {
    a + 2
}

fn vec_add_two(a: Vec<i32>) -> Vec<i32> {
    a.iter()
        .inspect(|x| println!("Got: {}", x))
        .map(|x| x + 2)
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_vec() {
        let a = vec![1, 2, 3];
        let b = vec_add_two(a);
        println!("{:?}", b);
        assert_eq!(b, vec![3, 4, 5]);
    }

    #[test]
    fn it_works() {
        assert_eq!(add_two(2), 4);
    }
}
