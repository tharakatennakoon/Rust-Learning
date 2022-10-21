use std::collections::HashMap;
use std::collections::BTreeSet;

pub fn first_uniq_char(s: String) -> i32 {
    let mut hm:HashMap<char, usize> = HashMap::new();
    let mut bts:BTreeSet<usize> = BTreeSet::new();

    for (index, c) in s.chars().enumerate() {
        let cs = hm.get(&c);

        match cs {
            Some(i) => {
                bts.remove(i);
            },
            None => {
                hm.insert(c, index);
                bts.insert(index);
            }
        }
    }
    
    match bts.iter().next() {
        Some(i) => return *i as i32,
        None => return -1
    };
}

struct CountChar {
    first_index:usize,
    char_count:i32
}

impl Ord for CountChar {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.first_index.cmp(&other.first_index)
    }
}

impl PartialOrd for CountChar {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.first_index.partial_cmp(&other.first_index) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.char_count.partial_cmp(&other.char_count)
    }
}

impl PartialEq for CountChar {
    fn eq(&self, other: &Self) -> bool {
        self.first_index == other.first_index
    }
}

impl Eq for CountChar {
    
}

pub fn first_uniq_char2(s: String) -> i32 {
    let mut hm:HashMap<char, CountChar> = HashMap::new();

    s.chars().enumerate().
    for_each(|(i,c)| {
        hm.entry(c).or_insert(CountChar{first_index:i, char_count:0}).char_count += 1;
    });

    hm.retain(|_,v| -> bool{
        v.char_count == 1
    });

    if hm.len() > 0 {
        let mut v:Vec<_> = hm.values().collect();

        v.sort();

        return v[0].first_index as i32;
    }
    
    return -1;
}

pub fn first_uniq_char3(s: String) -> i32 {
    let mut v = vec![0; 26];
    let ai = 'a' as usize;

    s.chars().for_each(|c| {
        v[c as usize - ai] += 1;
    });

    for (index, c) in s.chars().enumerate() {
        if v[c as usize - ai] == 1 {
            return index as i32;
        }
    }
    
    return -1;
}

#[cfg(test)]
mod test_first_unique_character {
    use super::first_uniq_char as test_fn;

    #[test]
    fn test1() {
        let s = "leetcode".to_owned();
        let expected = 0;

        let output = test_fn(s);

        assert_eq!(output, expected);
    }

    #[test]
    fn test2() {
        let s = "leetleetcode".to_owned();
        let expected = 8;

        let output = test_fn(s);

        assert_eq!(output, expected);
    }

    #[test]
    fn test3() {
        let s = "loveleetcode".to_owned();
        let expected = 2;

        let output = test_fn(s);

        assert_eq!(output, expected);
    }

    #[test]
    fn test4() {
        let s = "".to_owned();
        let expected = -1;

        let output = test_fn(s);

        assert_eq!(output, expected);
    }

    #[test]
    fn test5() {
        let s = "e".to_owned();
        let expected = 0;

        let output = test_fn(s);

        assert_eq!(output, expected);
    }

    #[test]
    fn test6() {
        let s = "ee".to_owned();
        let expected = -1;

        let output = test_fn(s);

        assert_eq!(output, expected);
    }
}

#[cfg(test)]
mod test_first_unique_character2 {
    use super::first_uniq_char2 as test_fn;

    #[test]
    fn test1() {
        let s = "leetcode".to_owned();
        let expected = 0;

        let output = test_fn(s);

        assert_eq!(output, expected);
    }

    #[test]
    fn test2() {
        let s = "leetleetcode".to_owned();
        let expected = 8;

        let output = test_fn(s);

        assert_eq!(output, expected);
    }

    #[test]
    fn test3() {
        let s = "loveleetcode".to_owned();
        let expected = 2;

        let output = test_fn(s);

        assert_eq!(output, expected);
    }

    #[test]
    fn test4() {
        let s = "".to_owned();
        let expected = -1;

        let output = test_fn(s);

        assert_eq!(output, expected);
    }

    #[test]
    fn test5() {
        let s = "e".to_owned();
        let expected = 0;

        let output = test_fn(s);

        assert_eq!(output, expected);
    }

    #[test]
    fn test6() {
        let s = "ee".to_owned();
        let expected = -1;

        let output = test_fn(s);

        assert_eq!(output, expected);
    }
}

#[cfg(test)]
mod test_first_unique_character3 {
    use super::first_uniq_char3 as test_fn;

    #[test]
    fn test1() {
        let s = "leetcode".to_owned();
        let expected = 0;

        let output = test_fn(s);

        assert_eq!(output, expected);
    }

    #[test]
    fn test2() {
        let s = "leetleetcode".to_owned();
        let expected = 8;

        let output = test_fn(s);

        assert_eq!(output, expected);
    }

    #[test]
    fn test3() {
        let s = "loveleetcode".to_owned();
        let expected = 2;

        let output = test_fn(s);

        assert_eq!(output, expected);
    }

    #[test]
    fn test4() {
        let s = "".to_owned();
        let expected = -1;

        let output = test_fn(s);

        assert_eq!(output, expected);
    }

    #[test]
    fn test5() {
        let s = "e".to_owned();
        let expected = 0;

        let output = test_fn(s);

        assert_eq!(output, expected);
    }

    #[test]
    fn test6() {
        let s = "ee".to_owned();
        let expected = -1;

        let output = test_fn(s);

        assert_eq!(output, expected);
    }
}
