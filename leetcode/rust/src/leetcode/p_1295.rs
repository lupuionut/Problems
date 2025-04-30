// 1295. Find Numbers with Even Number of Digits
// ---------------------------------------------

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        fn is_even(mut n: i32) -> bool {
            let mut even = true;
            while n > 0 {
                even = !even;
                n /= 10;
            }
            even
        }
        let mut ans = 0;
        for i in 0..nums.len() {
            if is_even(nums[i]) {
                ans += 1;
            }
        }
        ans
    }
}
