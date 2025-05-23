// 3068. Find the Maximum Sum of Node Values
// -----------------------------------------

impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
        let mut profits = vec![];
        let mut total = 0i64;
        for num in nums {
            total += num as i64;
            profits.push((num as i64 ^ k as i64) - num as i64);
        }
        profits.sort_by(|a, b| b.cmp(&a));

        let mut i = 0;
        while i < profits.len() {
            if i + 1 >= profits.len() {
                break;
            }
            let a = profits[i];
            let b = profits[i + 1];
            if (a + b) > 0 {
                total += (a + b);
            } else {
                break;
            }
            i += 2;
        }
        total
    }
}
