pub fn reversor(input: &str) -> String {
    input.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(reversor(""), "");
    }

    #[test]
    fn test_single() {
        assert_eq!(reversor("a"), "a");
    }

    #[test]
    fn test_single_utf8() {
        assert_eq!(reversor("😁"), "😁");
    }

    #[test]
    fn test_hello() {
        assert_eq!(reversor("hello"), "olleh");
    }

    #[test]
    fn test_utf8() {
        assert_eq!(reversor("😁😂"), "😂😁");
    }
}