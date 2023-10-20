// 2540. Minimum Common Value
// --------------------------

impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = 0;

        while l < nums1.len() && r < nums2.len() {
            if nums1[l] == nums2[r] {
                return nums1[l];
            } else if nums1[l] < nums2[r] {
                l += 1;
            } else {
                r += 1;
            }
        }

        -1
    }
}
