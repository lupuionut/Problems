// 2640. Find the Score of All Prefixes of an Array
// ------------------------------------------------

impl Solution {
    pub fn find_prefix_score(nums: Vec<i32>) -> Vec<i64> {
        let mut prefix_sum = 0;
        let mut best = 0;
        let mut ans = vec![];

        nums.iter().for_each(|&n| {
            let n = n as i64;
            best = best.max(n);
            let current = best + n;
            prefix_sum += current;
            ans.push(prefix_sum);
        });

        ans
    }
}
