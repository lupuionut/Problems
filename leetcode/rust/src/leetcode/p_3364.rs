// 3364. Minimum Positive Sum Subarray
// -----------------------------------

impl Solution {
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let mut ans = i32::MAX;
        for i in 0..nums.len() {
            let mut total = 0;
            let n = (r as usize + i - 1).min(nums.len() - 1); 
            for j in i..=n {
                total += nums[j];
                if j >= i - 1 + l as usize && j < i + r as usize && total > 0 {
                    ans = ans.min(total);
                }
            }
        }

        if ans == i32::MAX {
            return -1;
        }
        ans
    }
}
