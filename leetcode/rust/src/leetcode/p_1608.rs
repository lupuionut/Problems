// 1608. Special Array With X Elements Greater Than or Equal X
// -----------------------------------------------------------

impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        let mut ans = -1;
        for i in 0..=1000 {
            let mut count = 0;
            for j in 0..nums.len() {
                if nums[j] >= i as i32 {
                    count += 1;
                }
            }
            if count == i as i32 {
                return count;
            }
        }
        ans
    }
}
