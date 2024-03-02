// 977. Squares of a Sorted Array
// ------------------------------

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut start_idx = 0;
        let mut start_val = nums[0].abs();

        nums.iter().enumerate().for_each(|(k, &v)| {
            if v.abs() < start_val {
                start_idx = k;
                start_val = v.abs();
            }
        });

        let mut ans = vec![];
        let mut l = start_idx as i32;
        let mut r = start_idx as i32;

        while l >= 0 || r < nums.len() as i32 {
            if l < 0 {
                ans.push(nums[r as usize] * nums[r as usize]);
                r += 1;
                continue;
            }
            if r == nums.len() as i32 {
                ans.push(nums[l as usize] * nums[l as usize]);
                l -= 1;
                continue;
            }
            if l == r {
                ans.push(nums[l as usize] * nums[l as usize]);
                l -= 1;
                r += 1;
                continue;
            }
            if nums[l as usize].abs() < nums[r as usize].abs() {
                ans.push(nums[l as usize] * nums[l as usize]);
                l -= 1;
            } else {
                ans.push(nums[r as usize] * nums[r as usize]);
                r += 1;
            }
        }

        ans
    }
}
