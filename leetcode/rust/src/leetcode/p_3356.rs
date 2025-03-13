// 3356. Zero Array Transformation II
// ----------------------------------

impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut l = 0;
        let mut r = queries.len() + 1;

        fn all_zero(q: usize, queries: &Vec<Vec<i32>>, nums: &Vec<i32>) -> bool {
            let mut delta = vec![0; nums.len() + 1];

            for i in 0..q {
                let l = queries[i][0] as usize;
                let r = queries[i][1] as usize;
                let v = queries[i][2];
                delta[l] += v;
                delta[r + 1] -= v;
            }

            let mut diff = 0;
            for i in 0..nums.len() {
                diff += delta[i];
                if diff < nums[i] {
                    return false;
                }
            }
            true
        }

        while l < r {
            let mid = (l + r) / 2;
            if all_zero(mid, &queries, &nums) {
                r = mid;
            } else {
                l = mid + 1;
            }
        }

        if l == queries.len() + 1 {
            return -1;
        }
        l as i32
    }
}
