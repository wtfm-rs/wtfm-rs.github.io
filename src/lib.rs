pub fn hello_world() -> String {
    "Hello, world!".to_string()
}

#[cfg(test)]
#[test]
fn test_hello_world() {
    assert_eq!(hello_world(), "Hello, world!");
}
