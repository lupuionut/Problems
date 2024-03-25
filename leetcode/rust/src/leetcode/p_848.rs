// 848. Shifting Letters
// ---------------------

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
        let mut ps = vec![0; shifts.len()];
        let mut chars = s.chars().collect::<Vec<_>>();
        let mut shifts = shifts;
        shifts.reverse();

        for i in 0..shifts.len() {
            if i > 0 {
                ps[i] = (ps[i - 1] + shifts[i]) % 26;
            } else {
                ps[i] = shifts[i] % 26;
            }
        }
        ps.reverse();

        for i in 0..chars.len() {
            let digit = 97 + (((chars[i] as i32 - 97) + ps[i]) % 26) as u32;
            let c = char::from_u32(digit).unwrap();
            chars[i] = c;
        }

        chars.iter().collect::<String>()
    }
}
