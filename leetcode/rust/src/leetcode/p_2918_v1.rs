// 2918. Minimum Equal Sum of Two Arrays After Replacing Zeros
// -----------------------------------------------------------

impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let mut sum1 = 0;
        let mut sum2 = 0;
        let mut zeros1 = 0;
        let mut zeros2 = 0;
        for i in 0..nums1.len() {
            sum1 += nums1[i] as i64;
            if nums1[i] == 0 {
                zeros1 += 1;
            }
        }
        for i in 0..nums2.len() {
            sum2 += nums2[i] as i64;
            if nums2[i] == 0 {
                zeros2 += 1;
            }
        }

        let ans = (sum1 + zeros1).max(sum2 + zeros2);
        if (zeros1 == 0 && sum1 != ans) || (zeros2 == 0 && sum2 != ans) {
            return -1;
        }
        ans
    }
}
