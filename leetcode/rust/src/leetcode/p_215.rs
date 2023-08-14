// 215. Kth Largest Element in an Array
// ------------------------------------

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let k = nums.len() - k as usize;

        fn quickselect(l: usize, r: usize, nums: &mut Vec<i32>, k: usize) -> i32 {
            let mut ptr = l;

            for i in l..r {
                if nums[i] <= nums[r] {
                    let tmp = nums[ptr];
                    nums[ptr] = nums[i];
                    nums[i] = tmp;
                    ptr += 1;
                }
            }
            let tmp = nums[ptr];
            nums[ptr] = nums[r];
            nums[r] = tmp;

            if k > ptr {
                return quickselect(ptr + 1, r, nums, k);
            } else if k < ptr {
                return quickselect(l, ptr - 1, nums, k);
            } else {
                return nums[ptr];
            }
        }

        quickselect(0, nums.len() - 1, &mut nums, k)
    }
}
