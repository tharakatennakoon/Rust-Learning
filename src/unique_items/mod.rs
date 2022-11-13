use std::{collections::HashSet, hash::Hash};

pub fn unique<T>(nums : &mut Vec<T>)
    where T : Ord + Copy + Hash {
    
    if nums.is_empty(){
        return;
    }

    let mut num_set:HashSet<T> = HashSet::new();

    for (index, num) in nums.iter_mut().enumerate() {
        if num_set.contains(num) {
            nums.remove(index);
        }
    }


}

#[cfg(test)]
mod test_unique {
    use super::unique as test_fn;

    #[test]
    fn test_i32_empty(){
        let mut nums:Vec<i32> = vec![];

        let expected:Vec<i32> = vec![];

        test_fn(&mut nums);

        assert_eq!(nums, expected);

    }

    #[test]
    fn test_i32_unique(){
        let nums:Vec<i32> = vec![1,2,3];

        let expected:Vec<i32> = vec![1,2,3];

        test_fn(&mut &nums);

        assert_eq!(nums, expected);

    }

    #[test]
    fn test_i32_duplicate(){
        let nums:Vec<i32> = vec![1, 1, 2, 3];

        let expected:Vec<i32> = vec![1, 2, 3];

        test_fn(&mut &nums);

        assert_eq!(nums, expected);

    }

    #[test]
    fn test_i8_empty(){
        let nums:Vec<i8> = vec![];

        let expected:Vec<i8> = vec![];

        test_fn(&mut &nums);

        assert_eq!(nums, expected);

    }
}
