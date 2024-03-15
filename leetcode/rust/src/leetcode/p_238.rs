// 238. Product of Array Except Self
// ---------------------------------

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut zero_position = None;
        let mut zero_found = 0;
        let mut total_product = 1;

        nums.iter().enumerate().for_each(|(k, &n)| {
            if n == 0 {
                zero_found += 1;
                zero_position = Some(k);
            } else {
                total_product *= n;
            }
        });

        let mut ans = vec![0; nums.len()];
        if zero_found > 1 {
            return ans;
        }

        nums.iter().enumerate().for_each(|(k, &n)| {
            if let Some(position) = zero_position {
                if position == k {
                    ans[k] = total_product;
                }
            } else {
                ans[k] = total_product / n;
            }
        });

        ans
    }
}
