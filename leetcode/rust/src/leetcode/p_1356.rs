// 1356. Sort Integers by The Number of 1 Bits
// -------------------------------------------

impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        fn calculate(mut n: i32) -> i32 {
            let mut ones = 0;
            while n > 0 {
                if n & 1 == 1 {
                    ones += 1;
                }
                n = n >> 1;
            }

            ones
        }

        let mut ans = arr
            .iter()
            .map(|&n| (calculate(n), n))
            .collect::<Vec<(i32, i32)>>();

        ans.sort_by(|a, b| {
            if a.0 == b.0 {
                a.1.cmp(&b.1)
            } else {
                a.0.cmp(&b.0)
            }
        });

        ans.iter().map(|p| p.1).collect::<Vec<i32>>()
    }
}
