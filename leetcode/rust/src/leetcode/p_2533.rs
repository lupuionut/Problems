// 2553. Separate the Digits in an Array
// -------------------------------------

impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];

        nums.iter().for_each(|&n| {
            let mut n = n;
            let mut temp = vec![];
            while n != 0 {
                temp.push(n % 10);
                n = n / 10;
            }
            temp.iter().rev().for_each(|&n| {
                ans.push(n);
            });
        });

        ans
    }
}
