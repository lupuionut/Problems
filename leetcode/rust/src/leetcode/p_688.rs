// 688. Knight Probability in Chessboard
// -------------------------------------

impl Solution {
    pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
        let mut directions = [
            (2, 1),
            (2, -1),
            (1, 2),
            (1, -2),
            (-2, 1),
            (-2, -1),
            (-1, 2),
            (-1, -2),
        ];
        let mut cache = vec![vec![vec![-1.0; (k + 1) as usize]; n as usize]; n as usize];

        fn visit(
            n: i32,
            k: i32,
            row: i32,
            column: i32,
            cache: &mut Vec<Vec<Vec<f64>>>,
            directions: &[(i32, i32)],
        ) -> f64 {
            if row < 0 || row >= n || column < 0 || column >= n {
                return 0.0;
            }

            if k == 0 {
                return 1.0;
            }

            if cache[row as usize][column as usize][k as usize] != -1.0 {
                return cache[row as usize][column as usize][k as usize];
            }

            let mut prob = 0.0;
            for (dx, dy) in directions {
                prob += (visit(n, k - 1, row + dx, column + dy, cache, directions) / 8.0);
            }
            cache[row as usize][column as usize][k as usize] = prob;
            cache[row as usize][column as usize][k as usize]
        }

        visit(n, k, row, column, &mut cache, &directions)
    }
}
