// 1458. Max Dot Product of Two Subsequences
// -----------------------------------------

impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut cache = vec![vec![i32::MIN; nums2.len()]; nums1.len()];

        let nums1_max = *nums1.iter().max().unwrap();
        let nums1_min = *nums1.iter().min().unwrap();
        let nums2_max = *nums2.iter().max().unwrap();
        let nums2_min = *nums2.iter().min().unwrap();

        if nums1_max < 0 && nums2_min > 0 {
            return nums1_max * nums2_min;
        }

        if nums1_min > 0 && nums2_max < 0 {
            return nums1_min * nums2_max;
        }

        fn get_max(
            i1: usize,
            i2: usize,
            nums1: &Vec<i32>,
            nums2: &Vec<i32>,
            cache: &mut Vec<Vec<i32>>,
        ) -> i32 {
            if i1 == nums1.len() || i2 == nums2.len() {
                return 0;
            }

            if cache[i1][i2] != i32::MIN {
                return cache[i1][i2];
            }

            let mut best = i32::MIN;
            let mut res = get_max(i1 + 1, i2 + 1, nums1, nums2, cache);
            best = best.max(res + nums1[i1] * nums2[i2]);
            res = get_max(i1 + 1, i2, nums1, nums2, cache);
            best = best.max(res);
            res = get_max(i1, i2 + 1, nums1, nums2, cache);
            best = best.max(res);
            cache[i1][i2] = best;
            best
        }

        get_max(0, 0, &nums1, &nums2, &mut cache)
    }
}
