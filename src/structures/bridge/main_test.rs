use super::*;

#[test]
fn execute_test() {
    assert_eq!(
        ("( aiueo )".to_string(), "( kakikukeko )".to_string()),
        execute("aiueo", "kakikukeko")
    );
}
