// 216. Combination Sum III
// we have 2 options: take the current val, or don't take and move to next one

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();

        fn dfs(
            cur_val: i32,
            cur_values: &mut Vec<i32>,
            tn: i32,
            tk: i32,
            ans: &mut Vec<Vec<i32>>,
        ) -> () {
            if tk == 0 && tn == 0 {
                ans.push(cur_values.to_vec());
            }
            if cur_val > 9 || tn <= 0 || tk == 0 {
                return;
            }
            cur_values.push(cur_val);
            dfs(cur_val + 1, cur_values, tn - cur_val, tk - 1, ans);
            cur_values.pop();
            dfs(cur_val + 1, cur_values, tn, tk, ans);
        }

        let mut cur_values: Vec<i32> = Vec::new();
        dfs(1, &mut cur_values, n, k, &mut ans);
        ans
    }
}
