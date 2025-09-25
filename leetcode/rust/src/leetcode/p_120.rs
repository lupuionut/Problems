// 120. Triangle
// -------------

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut cache = vec![vec![]; triangle.len()];
        for i in 0..triangle.len() {
            for j in 0..triangle[i].len() {
                cache[i].push(None);
            }
        }

        fn dp(
            row: usize,
            col: usize,
            triangle: &Vec<Vec<i32>>,
            cache: &mut Vec<Vec<Option<i32>>>,
        ) -> i32 {
            if row + 1 == triangle.len() {
                return triangle[row][col];
            }
            if let Some(b) = cache[row][col] {
                return b;
            }
            let mut best = i32::MAX;
            best = best.min(triangle[row][col] + dp(row + 1, col, triangle, cache));
            best = best.min(triangle[row][col] + dp(row + 1, col + 1, triangle, cache));
            cache[row][col] = Some(best);
            best
        }

        dp(0, 0, &triangle, &mut cache)
    }
}
