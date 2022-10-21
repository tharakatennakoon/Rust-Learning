use std::char;
use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    //get chars to vector
    let s_slice = &s[..];
    let mut s_vec:Vec<char> = s_slice.chars().collect();
    s_vec.sort();

    let t_slice = &t[..];
    let mut t_vec:Vec<char> = t_slice.chars().collect();
    t_vec.sort();

    for (index, char) in s_vec.iter().enumerate() {
        if t_vec[index] != *char {
            return false;
        }
    }

    return true;
}

pub fn is_anagram_2(s:String, t:String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut c_map:HashMap<char, i32> = HashMap::new();

    s.chars().for_each(|c| *c_map.entry(c).or_insert(0) += 1);

    for c in t.chars() {
        if c_map.contains_key(&c) {
            if c_map[&c] > 0 {
                *c_map.entry(c).or_insert(0) -= 1
            }
            else {
                return false;
            }
        }
        else {
            return false;
        }
    }

    return true;
}

pub fn is_anagram_3(s:String, t:String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut c_map:HashMap<char, i32> = HashMap::new();

    s.chars().for_each(|c| *c_map.entry(c).or_insert(0) += 1);
    t.chars().for_each(|c| *c_map.entry(c).or_insert(0) -= 1);

    c_map.into_values().all(|v| v == 0)
}

#[cfg(test)]
mod test_valid_anagram {
    use super::is_anagram;

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

#[cfg(test)]
mod test_valid_anagram2 {
    use super::is_anagram_2;

    #[test]
    fn test1() {
        let s1 = String::from("anagram");
        let s2 = String::from("nagaram");

        assert_eq!(is_anagram_2(s1, s2), true);
    }

    #[test]
    fn test2() {
        let s1 = String::from("anagram");
        let s2 = String::from("nagarem");

        assert_eq!(is_anagram_2(s1, s2), false);
    }

    #[test]
    fn test3() {
        let s1 = String::from("anagram");
        let s2 = String::from("nagara");

        assert_eq!(is_anagram_2(s1, s2), false);
    }
}

#[cfg(test)]
mod test_valid_anagram3 {
    use super::is_anagram_3;

    #[test]
    fn test1() {
        let s1 = String::from("anagram");
        let s2 = String::from("nagaram");

        assert_eq!(is_anagram_3(s1, s2), true);
    }

    #[test]
    fn test2() {
        let s1 = String::from("anagram");
        let s2 = String::from("nagarem");

        assert_eq!(is_anagram_3(s1, s2), false);
    }

    #[test]
    fn test3() {
        let s1 = String::from("anagram");
        let s2 = String::from("nagara");

        assert_eq!(is_anagram_3(s1, s2), false);
    }
}
