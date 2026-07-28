// 3517. Smallest Palindromic Rearrangement I
// ------------------------------------------
impl Solution {
    pub fn smallest_palindrome(s: String) -> String {
        let mut f = vec![0; 26];
        s.chars().for_each(|c| {
            let k = c as usize - 97;
            f[k as usize] += 1;
        });
        let mut first = vec![];
        let mut half = '-';

        for i in 0..26 {
            let n = f[i] as usize;
            let h = n / 2;
            let k = char::from_u32((97 + i) as u32).unwrap();

            for j in 0..h {
                first.push(k);
            }
            if n % 2 == 1 {
                half = k;
            }
        }
        let second = first.clone().into_iter().rev().collect::<Vec<char>>();
        if half != '-' {
            first.push(half);
        }
        first.extend_from_slice(&second);
        first.iter().collect::<String>()
    }
}
