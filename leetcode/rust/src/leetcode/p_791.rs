// 791. Custom Sort String
// -----------------------

impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut order_chars = vec![0; 26];
        let mut s_chars = vec![0; 26];
        let mut ans = vec![];

        s.chars().for_each(|c| {
            let key = c as usize - 97;
            s_chars[key] += 1;
        });

        order.chars().for_each(|c| {
            let key = c as usize - 97;
            while s_chars[key] > 0 {
                ans.push(c);
                s_chars[key] -= 1;
            }
        });

        for i in 0..26 {
            let c = char::from_u32(97 + i as u32).unwrap();
            while s_chars[i] > 0 {
                ans.push(c);
                s_chars[i] -= 1;
            }
        }

        ans.into_iter().collect::<String>()
    }
}
