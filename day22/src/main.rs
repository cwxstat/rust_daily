fn build_vector(v: Vec<i32>) -> Vec<Vec<i32>> {
    let mut v2: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6]];
    v2.push(v);
    v2
}

fn main() {
    println!("Hello, world!");
    let mut v = vec![1, 2, 3];
    v.push(4);
    let mut v2: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6]];

    v2.push(vec![7]);
    println!("{:?}", v2[0][0]);
    println!("{:?}", v2.pop());

    let index = v2.binary_search(&vec![4, 5, 6]);
    match index {
        Ok(index) => println!("Found at index {}", index),
        Err(index) => println!("Not found, but would be inserted at index {}", index),
    }
    println!("{:?}", index);

    for i in v2.iter_mut() {
        i.push(10);
        println!("{:?}", i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = build_vector(vec![1, 2, 3]);
        assert_eq!(v.len(), 3);
    }
}
