fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    return true;
}

#[cfg(test)]
mod tests_valid_anagram {
    use crate::valid_anagram::is_anagram;

    #[test]
    fn test1() {
        let s1 = String::from("anagram");
        let s2 = String::from("nagaram");

        assert_eq!(is_anagram(s1, s2), true);
    }

    #[test]
    fn test2() {
        let s1 = String::from("anagram");
        let s2 = String::from("nagarem");

        assert_eq!(is_anagram(s1, s2), false);
    }

    #[test]
    fn test3() {
        let s1 = String::from("anagram");
        let s2 = String::from("nagara");

        assert_eq!(is_anagram(s1, s2), false);
    }
}
