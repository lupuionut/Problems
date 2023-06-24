// 976. Largest Perimeter Triangle
// -------------------------------

impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut ans = 0;
        nums.sort();

        for i in 0..nums.len() - 2 {
            if nums[i] + nums[i + 1] > nums[i + 2] {
                ans = ans.max(nums[i] + nums[i + 1] + nums[i + 2]);
            }
        }

        ans
    }
}
