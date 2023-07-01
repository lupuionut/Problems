// 1984. Minimum Difference Between Highest and Lowest of K Scores
// ---------------------------------------------------------------

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut windows = nums.windows(k as usize);
        let mut ans = 1_000_000;

        while let Some(frame) = windows.next() {
            let difference = frame[(k - 1) as usize] - frame[0];
            ans = ans.min(difference);
        }

        ans
    }
}
