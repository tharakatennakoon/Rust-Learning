//https://leetcode.com/problems/two-sum/

use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut ret: Vec<i32> = Vec::new();
    let mut diff_map: HashMap<i32,i32> = HashMap::new();

    for (index, value) in nums.iter().enumerate(){
        if diff_map.contains_key(&value) {
            ret.push(diff_map[&value]);
            ret.push(index as i32);
            break;
        }

        let _dif = target - value;
        diff_map.insert(_dif, index as i32);
    }

    return ret;
}

#[cfg(test)]
mod test_twosum {
    use super::two_sum;

    #[test]
    fn test1()
    {
        let input:Vec<i32> = vec![2,7,11,15];
        let ret:Vec<i32> = two_sum(input, 9);

        assert_eq!(ret.len(), 2);
        assert_eq!(ret[0], 0);
        assert_eq!(ret[1], 1);
    }

    #[test]
    fn test2()
    {
        let input:Vec<i32> = vec![3,2,4];
        let ret:Vec<i32> = two_sum(input, 6);

        assert_eq!(ret.len(), 2);
        assert_eq!(ret[0], 1);
        assert_eq!(ret[1], 2);
    }
}