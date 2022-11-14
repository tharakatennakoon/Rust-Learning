use std::fmt::Display;

pub fn print<T:Display + ?Sized>(s:&T) {
    println!("{}", s);
}

// slow due to internal memory copy
pub fn print_1<T:ToString + ?Sized>(s:&T) {
    println!("{}", s.to_string());
}

pub fn print_2<T:AsRef<str> + ?Sized>(s:&T) {
    println!("{}", s.as_ref());
}


#[cfg(test)]
mod test_print {
    use super::print as test_fn;

    #[test]
    fn test_1() {
        test_fn("test this");
    }

    #[test]
    fn test_2() {
        let s = "test this".to_owned();

        test_fn(&s);
    }

    #[test]
    fn test_3() {
        let s = "test this".to_owned();

        test_fn(&s);
    }

    #[test]
    fn test_4() {
        let s = "test this".to_owned();

        test_fn(s.as_str());
    }
}

#[cfg(test)]
mod test_print_1 {
    use super::print_1 as test_fn;

    #[test]
    fn test_1() {
        test_fn("test this");
    }

    #[test]
    fn test_2() {
        let s = "test this".to_owned();

        test_fn(&s);
    }

    #[test]
    fn test_3() {
        let s = "test this".to_owned();

        test_fn(&s);
    }

    #[test]
    fn test_4() {
        let s = "test this".to_owned();

        test_fn(s.as_str());
    }
}

#[cfg(test)]
mod test_print_2 {
    use super::print_2 as test_fn;

    #[test]
    fn test_1() {
        test_fn("test this");
    }

    #[test]
    fn test_2() {
        let s = "test this".to_owned();

        test_fn(&s);
    }

    #[test]
    fn test_3() {
        let s = "test this".to_owned();

        test_fn(&s);
    }

    #[test]
    fn test_4() {
        let s = "test this".to_owned();

        test_fn(s.as_str());
    }
}