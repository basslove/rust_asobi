use super::*;

#[test]
fn execute_test() {
    let result = execute();
    assert_eq!(
        (
            "Request accepted - v = 3".to_string(),
            "".to_string(),
            "".to_string()
        ),
        result
    );
}
