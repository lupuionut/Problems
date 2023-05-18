// 923. 3Sum With Multiplicity

impl Solution {
    pub fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
        let mut cache: Vec<Vec<Vec<i32>>> =
            vec![vec![vec![-1; (target + 1) as usize]; 4]; arr.len()];

        fn dfs(xs: &[i32], idx: i32, remain: i32, t: i32, cache: &mut Vec<Vec<Vec<i32>>>) -> i32 {
            if remain == 0 {
                if t == 0 {
                    return 1;
                } else {
                    return 0;
                }
            }
            if idx >= (xs.len() as i32) || t < 0 {
                return 0;
            }
            let a: usize = idx as usize;
            let b: usize = remain as usize;
            let c: usize = t as usize;
            if cache[a][b][c] != -1 {
                return cache[a][b][c];
            }
            cache[a][b][c] = dfs(xs, idx + 1, remain - 1, t - xs[idx as usize], cache) % 1000000007;
            cache[a][b][c] += dfs(xs, idx + 1, remain, t, cache) % 1000000007;
            cache[a][b][c] % 1000000007
        }

        dfs(&arr, 0, 3, target, &mut cache)
    }
}
