fn main() {

    // Define 4x4 Array
    let arr = [
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        [9, 10, 11, 12],
        [13, 14, 15, 16],
    ];
    // Define 4x4 Vector
    let vec = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16],
    ];

    // Add 1 to each element in the array
    let arr = arr.iter().map(|row| {
        row.iter().map(|col| {
            col + 1
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    // Print Array
    println!("Array: {:?}", arr);
    // Print Vector
    println!("Vector: {:?}", vec);
}
