// 2553. Separate the Digits in an Array
// -------------------------------------
impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        fn digits(n: i32, d: i32, ans: &mut Vec<i32>) {
            if d == 0 {
                return;
            }
            ans.push(n / d);
            digits(n % d, d / 10, ans);
        }

        for i in 0..nums.len() {
            let mut d = 1;
            if nums[i] >= 100000 {
                d = 100000;
            } else if nums[i] >= 10000 {
                d = 10000;
            } else if nums[i] >= 1000 {
                d = 1000;
            } else if nums[i] >= 100 {
                d = 100;
            } else if nums[i] >= 10 {
                d = 10;
            }
            digits(nums[i], d, &mut ans);
        }
        ans
    }
}
