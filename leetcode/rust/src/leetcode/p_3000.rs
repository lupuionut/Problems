// 3000. Maximum Area of Longest Diagonal Rectangle
// ------------------------------------------------

impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        let mut max_area = 0;
        let mut max_diag = 0;
        dimensions.iter().for_each(|d| {
            let diagonal = d[0] * d[0] + d[1] * d[1];
            if diagonal == max_diag {
                if d[0] * d[1] > max_area {
                    max_area = d[0] * d[1];
                }
            } else if diagonal > max_diag {
                max_diag = diagonal;
                max_area = d[0] * d[1];
            }
        });
        max_area
    }
}
