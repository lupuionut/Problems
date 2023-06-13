// 1573. Number of Ways to Split a String
// --------------------------------------

impl Solution {
    pub fn num_ways(s: String) -> i32 {
        let n = s.len() as i32;
        let modulo: u64 = 1_000_000_007;
        let mut ones = s.chars().enumerate().fold(vec![], |mut acc, (i, c)| {
            if c == '1' {
                acc.push(i);
            }
            acc
        });

        if ones.len() == 0 {
            return (((n - 2) as u64 % modulo * (n - 1) as u64 / 2 as u64 % modulo) % modulo)
                as i32;
        }

        if ones.len() % 3 != 0 {
            return 0;
        }

        let k = ones.len() / 3;
        let first = k - 1;
        let second = 2 * k - 1;
        ((ones[first + 1] - ones[first]) as u64 % modulo * (ones[second + 1] - ones[second]) as u64
            % modulo) as i32
    }
}
