// 1838. Frequency of the Most Frequent Element
// --------------------------------------------

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();

        let mut left = 0;
        let mut ans = 1;
        let mut cost = 0;

        for right in 1..nums.len() {
            cost += (nums[right] - nums[right - 1]) * ((right - left) as i32);
            while cost > k {
                cost -= nums[right] - nums[left];
                left += 1;
            }
            ans = ans.max(right - left + 1);
        }

        ans as i32
    }
}
