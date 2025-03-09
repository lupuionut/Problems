// 3208. Alternating Groups II
// ---------------------------

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut last = -1;
        let mut strike = 0;
        let mut head = 0;
        let mut tail = 0;

        while tail < colors.len() {
            if last == colors[head % colors.len()] {
                strike = 0;
            }
            last = colors[head % colors.len()];
            strike += 1;

            if (head as i32 - tail as i32 + 1).abs() == k {
                if strike >= k {
                    ans += 1;
                }
                tail += 1;
            }
            head += 1;
        }

        ans
    }
}
