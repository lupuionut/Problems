// 77. Combinations
// ----------------

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        fn backtrack(
            from: usize,
            to: usize,
            level: usize,
            current_comb: &mut Vec<i32>,
            ans: &mut Vec<Vec<i32>>,
        ) {
            if current_comb.len() == level {
                ans.push(current_comb.clone());
                return;
            }
            for n in from..=to {
                current_comb.push(n as i32);
                backtrack(n + 1, to, level, current_comb, ans);
                current_comb.pop();
            }
        }

        backtrack(1, n as usize, k as usize, &mut vec![], &mut ans);
        ans
    }
}
