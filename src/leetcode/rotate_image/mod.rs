// https://leetcode.com/problems/rotate-image/

pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let h = matrix.len();

    for y in 0 .. h / 2 {
        let x_off = y;
        let w = h - y * 2;

        for x in 0 .. w - 1 {
            let temp = matrix[y][x + x_off];

            matrix[y]                  [x_off + x]               = matrix[y + w - 1 - (x % w)][x_off];
            matrix[y + w - 1 - (x % w)][x_off]                   = matrix[y + w - 1][x_off + w - 1 - (x % w)];
            matrix[y + w - 1]          [x_off + w - 1 - (x % w)] = matrix[y + (x % w)][x_off + w - 1];
            matrix[y + x % w]          [x_off + w - 1]           = temp;
        }
    }
}

#[cfg(test)]
mod test_rotate {
    use super::rotate as test_fn;

    #[test]
    fn test_2_by_2(){
        let mut matrix = vec![
            vec![1,2], 
            vec![3,4]];

        let matrix_expected = vec![
            vec![3,1], 
            vec![4,2]];

        test_fn(&mut matrix);

        assert_eq!(matrix_expected, matrix);
    }

    #[test]
    fn test_3_by_3(){
        let mut matrix = vec![
            vec![1,2,3], 
            vec![4,5,6], 
            vec![7,8,9]];
        
        let matrix_expected = vec![
            vec![7,4,1], 
            vec![8,5,2], 
            vec![9,6,3]];

        test_fn(&mut matrix);

        assert_eq!(matrix_expected, matrix);
    }

    #[test]
    fn test_4_by_4(){
        //  1   2   3   4
        //  5   6   7   8
        //  9  10  11  12
        // 13  14  15  `16

        // 13   9   5   1
        // 14  10   6   2
        // 15  11   7   3
        // 16  12   8   4

        let mut matrix = vec![
            vec![ 1, 2, 3, 4],
            vec![ 5, 6, 7, 8], 
            vec![ 9,10,11,12], 
            vec![13,14,15,16]];

        let matrix_expected = vec![
            vec![13, 9, 5, 1], 
            vec![14,10, 6, 2], 
            vec![15,11, 7, 3], 
            vec![16,12, 8, 4]];

        test_fn(&mut matrix);

        assert_eq!(matrix_expected, matrix);
    }

    #[test]
    fn test_5_by_5(){
        //  1   2   3   4   5
        //  6   7   8   9  10
        // 11  12  13  14  15
        // 16  17  18  19  20
        // 21  22  23  24  25

        // 21  16  11   6   1
        // 22  17  12   7   2
        // 23  18  13   8   3
        // 24  19  14   9   4
        // 25  20  15   10  5

        let mut matrix = vec![
            vec![ 1, 2, 3, 4, 5], 
            vec![ 6, 7, 8, 9,10],
            vec![11,12,13,14,15], 
            vec![16,17,18,19,20],
            vec![21,22,23,24,25]];

        let matrix_expected = vec![
            vec![21,16,11, 6, 1],
            vec![22,17,12, 7, 2],
            vec![23,18,13, 8, 3],
            vec![24,19,14, 9, 4],
            vec![25,20,15,10, 5]
        ];

        test_fn(&mut matrix);

        assert_eq!(matrix_expected, matrix);
    }
}