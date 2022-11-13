use std::ops::Add;
use std::ops::Div;
use num_traits::FromPrimitive;
use num_traits::Num;

pub fn median<T>(mut num_array : Vec<T> ) -> T 
    // "Num" : This trait only allow numerical data tupes
    // "FromPrimitive" : used to convert 2 to T
    // find different between PartialOrd and Ord
    where T: Add<Output=T> + Div<Output=T> + Copy + PartialOrd + Num + FromPrimitive {
    let len = num_array.len();

    //num_array.sort(); // This cannot be used as float types does not implement Ord trait
    num_array.sort_by(|a,b| a.partial_cmp(b).unwrap());

    if len%2 != 0 {
        let index = len / 2;
        let median = num_array[index];
        return median;
    }
    else {
        let index1 = len / 2;
        let index2 = index1 - 1;
        
        let median = (num_array[index1] + num_array[index2]) / T::from_i32(2).unwrap();
        return median;
    }
}

#[cfg(test)]
mod test_median {
    use super::median as test_fn;

    #[test]
    fn test_i32_vector_odd() {
        let num = vec![1,2,3];
        let expected = 2;

        let output = test_fn(num);

        assert_eq!(output, expected);
    }

    #[test]
    fn test_i32_vector_even() {
        let num = vec![1,2,3,4];
        let expected = 2;

        let output = test_fn(num);

        assert_eq!(output, expected);
    }

    #[test]
    fn test_f64_vector_odd() {
        let num = vec![1.0,2.0,3.0];
        let expected = 2.0;

        let output = test_fn(num);

        assert_eq!(output, expected);
    }

    #[test]
    fn test_f64_vector_even() {
        let num = vec![1.0,2.0,3.0,4.0];
        let expected = 2.5;

        let output = test_fn(num);

        assert_eq!(output, expected);
    }
}