// 1291. Sequential Digits
// -----------------------
impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut ans = vec![];

        fn dp(last: i32, curr: i32, ans: &mut Vec<i32>, low: i32, high: i32) {
            if last >= 9 {
                return;
            }
            let mut curr = curr;
            let next = last + 1;
            curr *= 10;
            curr += next;

            if curr >= low && curr <= high {
                ans.push(curr);
            }
            dp(next, curr, ans, low, high);
        }

        for i in 1..9 {
            dp(i, i, &mut ans, low, high);
        }
        ans.sort();
        ans
    }
}
