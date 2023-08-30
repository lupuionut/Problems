// 2366. Minimum Replacements to Sort the Array
// --------------------------------------------

impl Solution {
    pub fn minimum_replacement(nums: Vec<i32>) -> i64 {
        let mut operations = 0;
        let mut target = *nums.last().unwrap();

        for i in (0..nums.len() - 1).rev() {
            if nums[i] > target {
                let q = nums[i] / target;
                let r = nums[i] % target;
                if r != 0 {
                    operations += q as i64;
                    target = nums[i] / (q + 1);
                } else {
                    operations += (q - 1) as i64;
                }
            } else {
                target = nums[i];
            }
        }

        operations
    }
}
