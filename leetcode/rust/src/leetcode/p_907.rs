// 907. Sum of Subarray Minimums
// -----------------------------

impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let mut sum: i64 = 0;
        let mut mods: i64 = 1_000_000_007;
        let mut stack: Vec<(usize, i32)> = vec![];

        arr.iter().enumerate().for_each(|(i, &n)| {
            while stack.len() > 0 && n < stack.last().unwrap().1 {
                let (j, m) = stack.pop().unwrap();
                let right = (i - j) as i32;
                let left = if stack.len() > 0 {
                    let (k, _) = stack.last().unwrap();
                    (j - k) as i32
                } else {
                    (j + 1) as i32
                };
                sum += ((right as i64 * left as i64 * m as i64) % mods);
                sum %= mods;
            }
            stack.push((i, n));
        });

        for i in 0..stack.len() {
            let (j, n) = stack[i];
            let left = if i > 0 {
                (j - stack[i - 1].0) as i32
            } else {
                (j + 1) as i32
            };
            let right = (arr.len() - j) as i32;
            sum += ((right as i64 * left as i64 * n as i64) % mods);
            sum %= mods;
        }

        (sum % mods) as i32
    }
}
