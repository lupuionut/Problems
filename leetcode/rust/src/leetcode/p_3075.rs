// 3075. Maximize Happiness of Selected Children
// ---------------------------------------------

impl Solution {
    pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
        happiness.sort();
        let mut step = 0;
        let mut ans = 0;

        while step < k && happiness.len() > 0 {
            let n = happiness.pop().unwrap();
            if n <= step {
                break;
            }
            ans += (n - step) as i64;
            step += 1;
        }

        ans
    }
}
