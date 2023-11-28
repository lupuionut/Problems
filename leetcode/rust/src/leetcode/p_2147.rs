// 2147. Number of Ways to Divide a Long Corridor
// ----------------------------------------------

impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        let mut ans: i64 = 1;
        let mut total_seats = 0;
        let mut curr = 0;
        let mut last_group = 0;

        corridor.chars().enumerate().for_each(|(i, c)| {
            if c == 'S' {
                total_seats += 1;
                curr += 1;
                if curr >= 2 {
                    if curr % 2 == 1 {
                        let diff = i - last_group;
                        ans = ans * (diff as i64);
                        ans %= 1_000_000_007;
                    } else {
                        last_group = i;
                    }
                }
            }
        });

        if total_seats < 2 || total_seats % 2 == 1 {
            return 0;
        }

        (ans % 1_000_000_007) as i32
    }
}
