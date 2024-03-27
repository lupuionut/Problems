// 713. Subarray Product Less Than K
// ---------------------------------

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut product = 1;
        let mut left = 0;

        for right in 0..nums.len() {
            product *= nums[right];
            if product < k {
                ans += (right - left + 1) as i32;
            } else {
                while left < right && product >= k {
                    product /= nums[left];
                    left += 1;
                }
                if product < k {
                    ans += (right - left + 1) as i32;
                }
            }
        }
        ans
    }
}
