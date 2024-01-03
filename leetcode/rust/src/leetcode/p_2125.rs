// 2125. Number of Laser Beams in a Bank
// -------------------------------------

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut last_row = 0;
        let mut ans = 0;

        bank.iter().for_each(|row| {
            let mut row_count = 0;
            row.chars().for_each(|c| {
                if c == '1' {
                    row_count += 1;
                }
            });
            if row_count != 0 {
                ans += row_count * last_row;
                last_row = row_count;
            }
        });

        ans
    }
}
