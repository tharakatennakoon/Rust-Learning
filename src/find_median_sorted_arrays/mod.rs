//https://leetcode.com/problems/median-of-two-sorted-arrays/

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let tot_len = nums1.len() + nums2.len();
    let index = tot_len / 2;

    let mut num1_pos = 0;
    let mut num2_pos = 0;
    let mut last_incrimented = 0;
    let mut pre_incrimented = 0;

   for _i in 0 .. index + 1 {
        pre_incrimented = last_incrimented;

        if num1_pos < nums1.len() && num2_pos < nums2.len() {
            let ele_1 = nums1[num1_pos];
            let ele_2 = nums2[num2_pos];
            
            if ele_1 < ele_2 {
                num1_pos += 1;
                last_incrimented = 0;
            }
            else {
                num2_pos += 1;
                last_incrimented = 1;
            }
        }
        else if num1_pos < nums1.len(){
            num1_pos += 1;
            last_incrimented = 0;
        }
        else {
            num2_pos += 1;
            last_incrimented = 1;
        }
    }

    if tot_len % 2 != 0 {
        if last_incrimented == 1 {
            return nums2[num2_pos - 1] as f64;
        }
        else {
            return nums1[num1_pos - 1] as f64;
        }
    }
    else {
        let n1 = if last_incrimented == 1 {
            num2_pos -= 1;
            nums2[num2_pos] as f64
        }
        else {
            num1_pos -= 1;
            nums1[num1_pos] as f64
        };
        
        let n2 = if pre_incrimented == 1 {
            nums2[num2_pos - 1] as f64
        }
        else {
            nums1[num1_pos - 1] as f64
        };
        
        return (n1 + n2) / 2.0   
    }
}

#[cfg(test)]
mod find_median_sorted_arrays {
    use super::find_median_sorted_arrays as test_fn;

    #[test]
    fn test_vecs_1() {
        let nums1:Vec<i32> = vec![];
        let nums2:Vec<i32> = vec![0];

        let expected:f64 = 0.0;

        let result = test_fn(nums1, nums2);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_vecs_2() {
        let nums1:Vec<i32> = vec![3];
        let nums2:Vec<i32> = vec![3];

        let expected:f64 = 3.0;

        let result = test_fn(nums1, nums2);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_vecs_3_same() {
        let nums1:Vec<i32> = vec![0,0];
        let nums2:Vec<i32> = vec![0];

        let expected:f64 = 0.0;

        let result = test_fn(nums1, nums2);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_vecs_4_same() {
        let nums1:Vec<i32> = vec![0, 0];
        let nums2:Vec<i32> = vec![0, 0];

        let expected:f64 = 0.0;

        let result = test_fn(nums1, nums2);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_1() {
        let nums1:Vec<i32> = vec![1, 3];
        let nums2:Vec<i32> = vec![2];

        let expected:f64 = 2.0;

        let result = test_fn(nums1, nums2);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_2() {
        let nums1:Vec<i32> = vec![1, 2];
        let nums2:Vec<i32> = vec![3, 4];

        let expected:f64 = 2.5;

        let result = test_fn(nums1, nums2);

        assert_eq!(expected, result);
    }
}