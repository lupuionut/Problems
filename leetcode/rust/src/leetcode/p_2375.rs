// 2375. Construct Smallest Number From DI String
// ----------------------------------------------

impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let mut chars: Vec<char> = pattern.chars().collect();
        let mut ans = vec![];
        let mut stack = vec![];

        for i in 0..=pattern.len() {
            stack.push(i + 1);

            while stack.len() > 0 && (i == pattern.len() || chars[i] == 'I') {
                let n = stack.pop().unwrap();
                let n = char::from_digit(n as u32, 10).unwrap();
                ans.push(n);
            }
        }

        ans.into_iter().collect::<String>()
    }
}
