fn append_this_is(input: &str) -> String {
    let mut result = String::from("this is ");
    result.push_str(input);
    result
}

fn append_this_is2(input: &mut String) {
    input.insert_str(0, "this is ");
}
fn main() {
    let input = "a test";
    let mut input2 = String::from("a test");

    let result = append_this_is(input);
    append_this_is2(&mut input2);
    println!("{}, {}", result, input2);
}
