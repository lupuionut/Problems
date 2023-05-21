// 2575. Find the Divisibility Array of a String

impl Solution {
    pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
        let mut remain: u64 = 0;
        let n: u64 = m as u64;
        let mut ans = vec![0; word.len()];
        let mut chars = word.chars();

        for (i, value) in chars.enumerate() {
            let digit = (value as i32) - 48;
            remain += digit as u64;
            if remain % n == 0 {
                ans[i] = 1;
            }
            remain = (remain % n) * 10;
        }

        ans
    }
}
