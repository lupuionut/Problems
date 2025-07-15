// 3136. Valid Word
// ----------------

impl Solution {
    pub fn is_valid(word: String) -> bool {
        if word.len() < 3 {
            return false;
        }
        let mut both = 0;
        for c in word.chars() {
            let k = c as u8;
            // numbers
            if (k >= 48 && k <= 57) {
                continue;
            }
            // vowel
            if k == 65
                || k == 69
                || k == 73
                || k == 79
                || k == 85
                || k == 97
                || k == 101
                || k == 105
                || k == 111
                || k == 117
            {
                both |= 1;
                continue;
            }
            // cons
            if (k >= 65 && k <= 90) || (k >= 97 && k <= 122) {
                both |= 2;
                continue;
            }

            return false;
        }
        both == 3
    }
}
