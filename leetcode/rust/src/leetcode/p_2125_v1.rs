// 2125. Number of Laser Beams in a Bank
// -------------------------------------

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut last = None;
        let mut ans = 0;
        bank.iter().for_each(|r| {
            let ones = r.chars().filter(|&c| c == '1').count() as i32;
            if let Some(v) = last {
                ans += (ones * v);
            }
            if ones > 0 {
                last = Some(ones);
            }
        });
        ans
    }
}
