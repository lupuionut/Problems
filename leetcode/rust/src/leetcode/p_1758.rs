// 1758. Minimum Changes To Make Alternating Binary String

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut current_z = '0';
        let mut current_o = '1';
        let mut zero_best = 0;
        let mut one_best = 0;

        for c in s.chars() {
            if c != current_z {
                zero_best += 1;
            } else {
                one_best += 1;
            }
            if current_z == '0' {
                current_z = '1';
                current_o = '0';
            } else {
                current_z = '0';
                current_o = '1';
            }
        }
        if zero_best < one_best {
            return zero_best;
        }
        one_best
    }
}
