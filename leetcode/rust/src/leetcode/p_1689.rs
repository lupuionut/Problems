// 1689. Partitioning Into Minimum Number Of Deci-Binary Numbers
// -------------------------------------------------------------
impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let mut ans = 0;

        n.chars().for_each(|c| {
            let x = c as usize - 48;
            if x > ans {
                ans += (x - ans);
            }
        });

        ans as i32
    }
}
