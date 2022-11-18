//https://leetcode.com/problems/top-k-frequent-elements/

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut out:Vec<i32> = vec![];

    let mut hm : HashMap<i32, i32> = HashMap::new();

    nums.iter().for_each(|num| *hm.entry(*num).or_insert(0) += 1);

    let mut vec: Vec<(i32, i32)> = hm.iter().map(|(num, count)| (*num, *count)).collect();

    vec.sort_by(|a, b| a.1.cmp(&b.1));

    for _i in 0..k {
        match vec.pop() {
            Some(element) => { out.push(element.0)},
            None => {}
        }
    }

    return out;
}

pub fn top_k_frequent_1(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut hm : HashMap<i32, i32> = HashMap::new();

    nums.iter().for_each(|num| *hm.entry(*num).or_insert(0) += 1);

    let mut vec: Vec<(i32, i32)> = hm.iter().map(|(num, count)| (*num, *count)).collect();

    vec.sort_by(|a, b| b.1.cmp(&a.1));

    let out:Vec<i32> = vec.iter().take(k as usize).map(|element| element.0).collect();

    return out;
}

#[cfg(test)]
mod test_top_k_frequent {
    use super::top_k_frequent as test_fn;

    #[test]
    fn test_empty_vec() {
        let nums:Vec<i32> = vec![];
        let expected:Vec<i32> = vec![];

        let result = test_fn(nums, 1);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_zero_k() {
        let nums:Vec<i32> = vec![1,2];
        let expected:Vec<i32> = vec![];

        let result = test_fn(nums, 0);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_1() {
        let nums:Vec<i32> = vec![1,1,1,2,2,3];
        let k = 2;
        let expected:Vec<i32> = vec![1,2];

        let result = test_fn(nums, k);

        assert_eq!(expected, result);
    }
}

#[cfg(test)]
mod test_top_k_frequent_1 {
    use super::top_k_frequent_1 as test_fn;

    #[test]
    fn test_empty_vec() {
        let nums:Vec<i32> = vec![];
        let expected:Vec<i32> = vec![];

        let result = test_fn(nums, 1);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_zero_k() {
        let nums:Vec<i32> = vec![1,2];
        let expected:Vec<i32> = vec![];

        let result = test_fn(nums, 0);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_1() {
        let nums:Vec<i32> = vec![1,1,1,2,2,3];
        let k = 2;
        let expected:Vec<i32> = vec![1,2];

        let result = test_fn(nums, k);

        assert_eq!(expected, result);
    }
}