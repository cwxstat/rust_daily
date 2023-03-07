use std::collections::HashSet;

fn add_to_set(set: &mut HashSet<String>, value: String) -> usize {
    set.insert(value);
    set.len()
}



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
        let mut v = build_vector(vec![1, 2, 3]);
        assert_eq!(v.len(), 3);
        for i in v.iter_mut() {
            i.push(10);
            println!("{:?}", i);
        }
        assert_eq!(v[0].len(), 4);
    }

    #[test]
    fn add_hs() {
        let mut hs = HashSet::new();
        let len  = add_to_set(&mut hs, "hello".to_string());
        assert_eq!(len, 1);
        assert_eq!(hs.len(), 1);
    }

    #[test]
    fn difference_intersection() {
        let mut hs1 = HashSet::new();
        let mut hs2 = HashSet::new();

        add_to_set(&mut hs1, "hello".to_string());
        add_to_set(&mut hs1, "bob".to_string());
        add_to_set(&mut hs2, "hello".to_string());
        add_to_set(&mut hs2, "susan".to_string());
        println!("{:?}", hs1.difference(&hs2).collect::<Vec<_>>());
        println!("{:?}", hs1.intersection(&hs2).collect::<Vec<_>>());
        println!("{:?}", hs1.symmetric_difference(&hs2).collect::<Vec<_>>());


    }
    #[test]
    fn h() {
        let mut hs1 = HashSet::new();
        let mut hs2 = HashSet::new();

        hs1.insert([[1,0,0],[0,1,0],[0,0,1]]);
        hs2.insert([[1,0,0],[0,1,0],[0,0,1]]);
        hs2.insert([[1,1,1],[0,1,0],[0,0,1]]);
        println!("{:?}", hs2.difference(&hs1).collect::<Vec<_>>());



    }
}
