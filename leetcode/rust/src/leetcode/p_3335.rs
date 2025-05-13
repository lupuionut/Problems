// 3335. Total Characters in String After Transformations I
// --------------------------------------------------------

impl Solution {
    pub fn length_after_transformations(s: String, mut t: i32) -> i32 {
        let mut letters = [0i64; 26];

        s.as_bytes().into_iter().for_each(|&b| {
            let key = b as usize - 97;
            letters[key] += 1;
        });
        while t > 0 {
            let mut temp = letters[25] % 1_000_000_007;
            for i in (0..=25).rev() {
                if i == 0 {
                    letters[0] += temp;
                    letters[1] += temp;
                } else {
                    letters[i] = letters[i - 1] % 1_000_000_007;
                    letters[i - 1] = 0;
                }
            }
            t -= 1;
        }

        (letters.iter().sum::<i64>() % 1_000_000_007) as i32
    }
}
