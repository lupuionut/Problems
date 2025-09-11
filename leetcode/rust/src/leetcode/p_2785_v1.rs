impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let mut ans = String::new();
        let mut vowels = vec![];
        
        s.chars().for_each(|c| {
            match c {
                'a' | 'A' | 'e' | 'E' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U' => {vowels.push(c)},
                _ => {}
            }
        });
        vowels.sort_by(|a,b| b.cmp(&a));
        s.chars().for_each(|c| {
            match c {
                'a' | 'A' | 'e' | 'E' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U' => ans.push(vowels.pop().unwrap()),
                _ => ans.push(c)
            }
        });

        ans
    }
}
