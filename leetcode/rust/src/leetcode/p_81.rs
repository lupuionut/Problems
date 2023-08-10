// 81. Search in Rotated Sorted Array II
// -------------------------------------

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let mut l = 0;
        let mut r = nums.len() - 1;

        while l <= r {
            let m = l + (r - l) / 2;

            if nums[m] == target {
                return true;
            }

            // if left side is sorted
            if nums[l] < nums[m] {
                if nums[l] <= target && target < nums[m] {
                    r = m - 1;
                } else {
                    l = m + 1;
                }
            // if left side is not sorted
            } else if nums[l] > nums[m] {
                if nums[m] < target && target <= nums[r] {
                    l = m + 1;
                } else {
                    r = m - 1;
                }
            } else {
                l += 1;
            }
        }
        false
    }
}
