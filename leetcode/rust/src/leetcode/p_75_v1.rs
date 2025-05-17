// 75. Sort Colors
// ---------------

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut i = 0;
        let mut l = 0;
        let mut r = nums.len() - 1;
        while r > 0 && i <= r {
            if nums[i] == 0 {
                let t = nums[l];
                nums[l] = nums[i];
                nums[i] = t;
                l += 1;
                i += 1;
            } else if nums[i] == 2 {
                let t = nums[r];
                nums[r] = nums[i];
                nums[i] = t;
                r -= 1;
            } else {
                i += 1;
            }
        }
    }
}
