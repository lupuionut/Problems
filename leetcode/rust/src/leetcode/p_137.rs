// 137. Single Number II
// ---------------------

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ans: i64 = 0;
        for i in 0..32 {
            let mut total = 0;
            for num in &nums {
                total += (num >> i) & 1;
            }
            if total % 3 != 0 {
                ans = ans | (1 << i);
            }
        }
        ans as i32
    }
}
