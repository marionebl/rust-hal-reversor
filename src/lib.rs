pub fn reversor(input: &str) -> String {
    input.chars().rev().collect()
}

#[test]
fn test_hello() {
    assert_eq!(reversor("hello"), "olleh");
}