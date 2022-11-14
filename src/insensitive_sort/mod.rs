
pub fn insensitive_sort<T:ToString + PartialOrd>(mut list:Vec<T>) -> Vec<T> {
    list.sort_by(|a,b|a.to_string().to_lowercase().partial_cmp(&b.to_string().to_lowercase()).unwrap());
    return list;
}

pub fn insensitive_sort_1<T:AsRef<str> + Ord>(mut list:Vec<T>) -> Vec<T> {

    list.sort_by_cached_key(|s| s.as_ref().to_lowercase());
    return list;
}

#[cfg(test)]
mod test_insensitive_sort {
    use super::insensitive_sort as test_fn;

    #[test]
    fn test_epmpty_vec() {
        let list : Vec<String> = vec![];
        let expected : Vec<String> = vec![];

        let actual = test_fn(list);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_1() {
        let list = vec!["a", "A"];
        let expected = vec!["a", "A"];

        let actual = test_fn(list);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_2() {
        let list = vec!["a", "b", "B", "A"];
        let expected = vec!["a", "A", "b", "B"];

        let actual = test_fn(list);

        assert_eq!(expected, actual);
    }
}

#[cfg(test)]
mod test_insensitive_sort_1 {
    use super::insensitive_sort_1 as test_fn;

    #[test]
    fn test_epmpty_vec() {
        let list : Vec<String> = vec![];
        let expected : Vec<String> = vec![];

        let actual = test_fn(list);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_1() {
        let list = vec!["a", "A"];
        let expected = vec!["a", "A"];

        let actual = test_fn(list);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_2() {
        let list = vec!["a", "b", "B", "A"];
        let expected = vec!["a", "A", "b", "B"];

        let actual = test_fn(list);

        assert_eq!(expected, actual);
    }
}
