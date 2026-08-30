// 2091. Removing Minimum and Maximum From Array
// ---------------------------------------------

impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        if n == 1 {
            return 1;
        }

        let mut min = (i32::MAX, 0);
        let mut max = (i32::MIN, 0);

        for i in 0..nums.len() {
            if nums[i] < min.0 {
                min = (nums[i], i);
            }
            if nums[i] > max.0 {
                max = (nums[i], i);
            }
        }

        let a = min.1.max(max.1) + 1;
        let b = n - min.1.min(max.1);
        let c = min.1.min(max.1) + (n - max.1.max(min.1)) + 1;

        (a.min(b)).min(c) as i32
    }
}
