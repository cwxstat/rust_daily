fn vec_vec() -> Vec<Vec<i32>> {
    #[rustfmt::skip]
    let  vec = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];
    vec
}

fn main() {
    let vec = vec_vec();
    println!("{:?}", vec);
}
