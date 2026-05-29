// 3300. Minimum Element After Replacement With Digit Sum
// ------------------------------------------------------
impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        fn reduce(mut n: i32) -> i32 {
            let mut ans = 0;
            while n >= 10 {
                ans += n % 10;
                n /= 10;
            }
            ans += n;
            ans
        }

        let mut min = i32::MAX;
        for i in 0..nums.len() {
            min = min.min(reduce(nums[i]));
        }
        min
    }
}
