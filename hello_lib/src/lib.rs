

/// This function returns a greeting.
/// 
/// Example:
/// ```
///     use hello_lib::hello;
/// 
///     assert_eq!(hello(), "Hello World");
/// ```
pub fn hello() -> String {
    return "Hello World".to_string()
}

#[cfg(test)]
mod tests {
    use super::hello;

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello World");
    }
}
