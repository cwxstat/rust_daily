use std::collections::HashMap;

fn create_hashmap() -> Box<HashMap<i32, String>> {
    let mut map = HashMap::new();
    map.insert(1, "hello".to_string());
    map.insert(2, "world".to_string());

    Box::new(map)
}

fn add_to_boxed_hashmap(box_map: &mut Box<HashMap<i32, String>>, key: i32, value: String) {
    for i in 0..10 {
        box_map.insert(i, format!("hello {}", i));
    }
    box_map.insert(key, value);
}

fn main() {
    let mut box_map = create_hashmap();
    add_to_boxed_hashmap(&mut box_map, 0, "foo".to_string());
    println!("{:?}", box_map);
}
