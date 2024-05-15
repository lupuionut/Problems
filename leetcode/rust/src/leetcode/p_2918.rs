// 2918. Minimum Equal Sum of Two Arrays After Replacing Zeros
// -----------------------------------------------------------

impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let mut sum1 = 0i64;
        let mut sum2 = 0i64;
        let mut z1 = 0i64;
        let mut z2 = 0i64;

        for i in 0..nums1.len() {
            sum1 += nums1[i] as i64;
            if nums1[i] == 0 {
                z1 += 1;
            }
        }

        for i in 0..nums2.len() {
            sum2 += nums2[i] as i64;
            if nums2[i] == 0 {
                z2 += 1;
            }
        }

        let target = (sum1 + z1).max(sum2 + z2);
        if (z1 == 0 && sum1 != target) || (z2 == 0 && sum2 != target) {
            return -1;
        }
        target
    }
}
