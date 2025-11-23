pub fn is_palindrome(s: &str) -> bool {
    let mut filtered = s.chars()
        .zip(s.chars().rev())
        .filter(|x| x.0 != x.1)
        .peekable();

    !filtered.peek().is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
        
    #[test]
    fn test_is_palindrome() {
        assert_eq!(true, is_palindrome(""));
        assert_eq!(false, is_palindrome("not a palindrome"));
        assert_eq!(true, is_palindrome("())(")); 
        assert_eq!(true, is_palindrome("a"));
        assert_eq!(true, is_palindrome("✅"));
        assert_eq!(true, is_palindrome("✅❌✅"));
        assert_eq!(false, is_palindrome("✅❌"));
    }
}
