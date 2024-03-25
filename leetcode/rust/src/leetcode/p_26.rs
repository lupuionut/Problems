// 26. Remove Duplicates from Sorted Array
// ---------------------------------------

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = 1;

        while right < nums.len() {
            if nums[left] != nums[right] {
                nums[left + 1] = nums[right];
                left += 1;
            }
            right += 1;
        }

        (left + 1) as i32
    }
}
