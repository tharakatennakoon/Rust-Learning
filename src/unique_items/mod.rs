use std::{collections::HashSet, hash::Hash};

pub fn unique<T>(nums : Vec<T>) -> Vec<T>
    where T : Ord + Copy + Hash {

    if nums.is_empty() || nums.len() == 1{
        return nums;
    }
    
    let mut num_set:HashSet<T> = HashSet::new();
    let mut unique_vec : Vec<T> = vec![];

    for num in nums {
        if num_set.contains(&num) {
            continue;
        }

        num_set.insert(num);
        unique_vec.push(num);
    }
    
    return unique_vec;
}

#[cfg(test)]
mod test_unique {
    use super::unique as test_fn;

    #[test]
    fn test_i32_empty(){
        let nums:Vec<i32> = vec![];

        let expected:Vec<i32> = vec![];

        let result = test_fn(nums);

        assert_eq!(result, expected);

    }

    #[test]
    fn test_i32_unique(){
        let nums:Vec<i32> = vec![1,2,3];

        let expected:Vec<i32> = vec![1,2,3];

        let result = test_fn(nums);

        assert_eq!(result, expected);

    }

    #[test]
    fn test_i32_duplicate(){
        let nums:Vec<i32> = vec![1, 1, 2, 3];

        let expected:Vec<i32> = vec![1, 2, 3];

        let result = test_fn(nums);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_i32_duplicate_unsorted() {
        let nums:Vec<i32> = vec![3, 1, 1, 2];

        let expected:Vec<i32> = vec![3, 1, 2];

        let result = test_fn(nums);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_i8_empty(){
        let nums:Vec<i8> = vec![];

        let expected:Vec<i8> = vec![];

        let result = test_fn(nums);

        assert_eq!(result, expected);
    }
}
