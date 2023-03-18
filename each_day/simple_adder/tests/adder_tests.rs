use simple_adder::add;

#[test]
fn test_add() {
    assert_eq!(add(1, 2), 3);
    assert_eq!(add(0, 0), 0);
    assert_eq!(add(-1, 2), 1);
    assert_eq!(add(1, -2), -1);
}