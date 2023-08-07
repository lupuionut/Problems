// 74. Search a 2D Matrix
// ----------------------

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut left = 0;
        let mut right = (m - 1) as i32;

        while left <= right {
            let mut middle = (left + right) / 2;
            if matrix[middle as usize][0] <= target && matrix[middle as usize][n - 1] >= target {
                let mut left = 0;
                let mut right = n as i32;
                let k = middle as usize;
                while left <= right {
                    let middle = (left + right) / 2;
                    if matrix[k][middle as usize] < target {
                        left = middle + 1;
                    } else {
                        right = middle - 1;
                    }
                }
                if matrix[k][left as usize] == target {
                    return true;
                }
                return false;
            } else {
                if matrix[middle as usize][0] < target {
                    left = middle + 1;
                } else {
                    right = middle - 1;
                }
            }
        }

        false
    }
}
