// 3350. Adjacent Increasing Subarrays Detection II
// ------------------------------------------------
impl Solution {
    pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
        let mut last = None;
        let mut streak = 0;
        let mut streaks = nums.clone();
        for i in 0..nums.len() {
            if let Some(n) = last {
                if nums[i] > n {
                    streak += 1;
                } else {
                    streak = 1;
                }
            } else {
                streak = 1;
            }
            streaks[i] = streak;
            last = Some(nums[i]);
        }

        let mut best = *streaks.iter().max().unwrap();
        best /= 2;

        for i in 0..streaks.len() {
            if (i < streaks[i] as usize) || (streaks[i] < best) {
                continue;
            }
            let p = i - streaks[i] as usize;
            if streaks[p] >= streaks[i] {
                best = best.max(streaks[i]);
            }
        }
        best
    }
}
