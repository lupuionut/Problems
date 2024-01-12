// 1704. Determine if String Halves Are Alike
// ------------------------------------------

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let mut first_part_vowels = 0;
        let mut second_part_vowels = 0;
        let mut half = s.len() / 2;
        s.chars().enumerate().for_each(|(k, v)| {
            let count = match v {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => 1,
                _ => 0,
            };

            if k < half {
                first_part_vowels += count;
            } else {
                second_part_vowels += count;
            }
        });

        first_part_vowels == second_part_vowels
    }
}
