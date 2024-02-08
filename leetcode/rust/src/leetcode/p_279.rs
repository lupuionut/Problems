// 279. Perfect Squares
// --------------------

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut squares = vec![];
        for i in 1..=n {
            let j = i * i;
            if j <= n {
                squares.push(j);
            }
        }
        let mut cache = vec![-1; (n + 1) as usize];
        Solution::dp(n, &squares, &mut cache)
    }

    pub fn dp(target: i32, squares: &Vec<i32>, cache: &mut Vec<i32>) -> i32 {
        if target == 0 {
            return 0;
        }
        if cache[target as usize] != -1 {
            return cache[target as usize];
        }
        let mut best = i32::MAX;
        for square in squares.iter() {
            let target = target - square;
            if target >= 0 {
                best = best.min(1 + Solution::dp(target, squares, cache));
            }
        }
        cache[target as usize] = best;
        cache[target as usize]
    }
}
