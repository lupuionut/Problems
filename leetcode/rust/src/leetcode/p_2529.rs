// 2529. Maximum Count of Positive Integer and Negative Integer
// ------------------------------------------------------------

impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let mut pos = 0;
        let mut neg = 0;
        nums.iter().for_each(|&n| {
            if n < 0 {
                neg += 1;
            } else if n > 0 {
                pos += 1;
            }
        });
        neg.max(pos)
    }
}
