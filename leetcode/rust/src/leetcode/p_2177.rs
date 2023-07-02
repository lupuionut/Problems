// 2177. Find Three Consecutive Integers That Sum to a Given Number
// ----------------------------------------------------------------

impl Solution {
    pub fn sum_of_three(num: i64) -> Vec<i64> {
        let mut ans = vec![];
        if num % 3 != 0 {
            return ans;
        }
        let m = num / 3;
        ans.push(m - 1);
        ans.push(m);
        ans.push(m + 1);

        ans
    }
}
