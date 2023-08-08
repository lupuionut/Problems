// 33. Search in Rotated Sorted Array
// ----------------------------------

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        let mut rotate_idx: i32 = 10_000;
        let n = nums.len() - 1;

        if nums[l] <= nums[r] {
            rotate_idx = 0;
        } else {
            while l <= r {
                let mid = (l + r) / 2;
                // either nums[mid] < nums[0], so the rotation happens in the left side
                // 40123
                //   ^
                // or nums[mid] > nums[0], so the rotation happens in the right side
                // 23401
                //   ^
                if nums[mid] < nums[0] {
                    rotate_idx = rotate_idx.min(mid as i32);
                    r = mid - 1;
                } else {
                    l = mid + 1;
                }
            }
        }

        let mut l = 0;
        let mut r = 0;

        if nums[rotate_idx as usize] <= target && target <= nums[n] {
            l = rotate_idx;
            r = n as i32;
        } else {
            l = 0;
            r = rotate_idx;
        }

        while l <= r {
            let mid = (l + r) / 2;
            if nums[mid as usize] == target {
                return mid;
            }
            if nums[mid as usize] < target {
                l += 1;
            } else {
                r -= 1;
            }
        }
        -1
    }
}
