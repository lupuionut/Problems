// 1718. Construct the Lexicographically Largest Valid Sequence
// ------------------------------------------------------------

impl Solution {
    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        let mut ans = vec![-1; (2 * n - 1) as usize];
        let mut used = 0;

        fn backtrack(i: usize, n: i32, ans: &mut Vec<i32>, used: &mut i32) -> bool {
            if i == ans.len() {
                return true;
            }

            for j in (1..=n).rev() {
                let position = 1 << j;
                if *used & position != 0 {
                    continue;
                }

                if j > 1 && ((j as usize + i) >= ans.len() || ans[j as usize + i] != -1) {
                    continue;
                }

                *used |= position;
                ans[i] = j;
                if j > 1 {
                    ans[i + j as usize] = j;
                }

                let mut k = i + 1;
                while k < ans.len() && ans[k] != -1 {
                    k += 1;
                }

                if backtrack(k, n, ans, used) {
                    return true;
                }

                *used ^= position;
                ans[i] = -1;
                if j > 1 {
                    ans[i + j as usize] = -1;
                }
            }

            false
        }
        backtrack(0, n, &mut ans, &mut used);
        ans
    }
}
