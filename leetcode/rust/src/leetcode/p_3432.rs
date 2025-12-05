// 3432. Count Partitions with Even Sum Difference
// -----------------------------------------------
impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        let mut ps = nums.clone();
        let n = nums.len();
        let mut ans = 0;
        for i in 1..n {
            ps[i] += ps[i - 1];
        }
        for i in 0..n - 1 {
            if (2 * ps[i] - ps[n - 1]) % 2 == 0 {
                ans += 1;
            }
        }
        ans
    }
}
