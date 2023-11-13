// 2785. Sort Vowels in a String
// -----------------------------

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let mut vowels = vec![];
        let mut word = vec![];
        s.chars().for_each(|c| match c.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => vowels.push(c),
            _ => (),
        });

        vowels.sort();
        let mut i = 0;
        s.chars().for_each(|c| match c.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                word.push(vowels[i]);
                i += 1;
            }
            _ => word.push(c),
        });

        word.iter().collect::<String>()
    }
}
