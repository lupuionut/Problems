// 1144. Decrease Elements To Make Array Zigzag
// --------------------------------------------

impl Solution {
    pub fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
        let mut nums1 = nums.clone();
        let mut nums2 = nums.clone();
        let mut diff1 = 0;
        let mut diff2 = 0;

        // even
        for i in 1..nums1.len() {
            if i % 2 == 0 {
                if nums1[i - 1] >= nums1[i] {
                    diff1 += (nums1[i - 1] - nums1[i] + 1);
                }
            } else {
                if nums1[i - 1] <= nums1[i] {
                    diff1 += (nums1[i] - nums1[i - 1] + 1);
                    nums1[i] = nums1[i - 1] - 1;
                }
            }
        }

        // odd
        for i in 1..nums2.len() {
            if i % 2 == 0 {
                if nums2[i] >= nums2[i - 1] {
                    diff2 += (nums2[i] - nums2[i - 1] + 1);
                    nums2[i] = nums2[i - 1] - 1;
                }
            } else {
                if nums2[i - 1] >= nums2[i] {
                    diff2 += (nums2[i - 1] - nums2[i] + 1);
                }
            }
        }

        diff1.min(diff2)
    }
}
