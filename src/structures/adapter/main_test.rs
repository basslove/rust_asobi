use super::*;

#[test]
fn execute_test() {
    assert_eq!((10, 120), execute(&ObjectX { a: 10, b: 120 }));
    assert_eq!((1, 2), execute(&ObjectY { m: 1, n: 2 }));
}
