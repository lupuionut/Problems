// 3318. Find X-Sum of All K-Long Subarrays I
// ------------------------------------------
use std::collections::BinaryHeap;
impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut l = 0;
        let mut counter = vec![0; 51];
        let mut ans = vec![];
        for r in 0..nums.len() {
            counter[nums[r] as usize] += 1;
            while (r - l + 1) > k as usize {
                counter[nums[l] as usize] -= 1;
                l += 1;
            }

            if r - l + 1 == k as usize {
                let mut h = BinaryHeap::new();
                for i in 0..counter.len() {
                    h.push((counter[i], i as i32));
                }
                let mut j = 0;
                let mut t = 0;
                while j < x {
                    if let Some(v) = h.pop() {
                        t += (v.0 * v.1);
                    }
                    j += 1;
                }
                ans.push(t);
            }
        }
        ans
    }
}
