// 2028. Find Missing Observations
// -------------------------------

impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        let sum = rolls.iter().fold(0, |mut acc, x| {
            acc += x;
            return acc;
        });
        let max = 6 * n;
        let wanted = (rolls.len() as i32 + n) * mean - sum;

        if wanted > max || wanted < n {
            return ans;
        }

        let av = wanted / n;
        let mut delta = wanted - (av * n);

        for i in 0..n {
            if delta > 0 {
                ans.push(av + 1);
                delta -= 1;
            } else {
                ans.push(av);
            }
        }
        ans
    }
}
