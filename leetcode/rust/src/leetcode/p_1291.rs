// 1291. Sequential Digits
// -----------------------

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut ans = vec![];

        for i in 1..9 {
            let mut curr = i;
            while curr <= high {
                if curr >= low {
                    ans.push(curr);
                }
                let last_digit = curr % 10;
                if last_digit == 9 {
                    break;
                }
                curr = (curr * 10) + (last_digit + 1);
            }
        }
        ans.sort();
        ans
    }
}
