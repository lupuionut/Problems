// 451. Sort Characters By Frequency
// ---------------------------------

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut freq = vec![(' ', 0); 62];
        s.chars().for_each(|c| {
            let cu = c as usize;
            let mut key = 0;
            // digits -> 0..9
            if cu >= 48 && cu <= 57 {
                key = cu - 48;
            // uppercase -> 10..35
            } else if cu >= 65 && cu <= 90 {
                key = cu - 65 + 10;
            // lowercase -> 36..61
            } else {
                key = cu - 97 + 36;
            }
            freq[key] = (c, freq[key].1 + 1);
        });

        let mut chars = freq.iter().filter(|(_, f)| *f > 0).collect::<Vec<_>>();
        chars.sort_by(|a, b| b.1.cmp(&a.1));
        let ans = chars
            .iter()
            .map(|(c, f)| String::from(*c).repeat(*f))
            .collect::<String>();
        ans
    }
}
