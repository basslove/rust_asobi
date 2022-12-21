use super::*;

#[test]
fn execute_test() {
    let result = execute();
    assert_eq!("Alice", result.name);
    assert_eq!(12, result.age);
    assert_eq!(Some("Student".to_string()), result.job);
}
