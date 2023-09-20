// 1658. Minimum Operations to Reduce X to Zero
// --------------------------------------------

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let target = nums.iter().sum::<i32>() - x;
        if target == 0 {
            return nums.len() as i32;
        }

        let mut ans = 0;
        let mut left = 0;
        let mut current_total = 0;

        for right in 0..nums.len() {
            current_total += nums[right];
            while current_total > target && left < right {
                current_total -= nums[left];
                left += 1;
            }
            if current_total == target {
                let d = (right - left + 1) as i32;
                ans = ans.max(d)
            }
        }

        if ans == 0 {
            -1
        } else {
            nums.len() as i32 - ans
        }
    }
}
