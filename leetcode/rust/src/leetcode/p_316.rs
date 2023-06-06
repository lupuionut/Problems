// 316. Remove Duplicate Letters
// -----------------------------
// Keep a stack with each char.
// When inserting a new char, check if existing chars in stack
// are higher than the existing one and if other occurences of that
// chars might appear later, if so, remove those chars from stack.

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut max_found_pos = vec![0; 26];
        let mut visited = vec![0; 26];
        let mut stack: Vec<usize> = Vec::new();
        let mut ans = "".to_owned();

        for (pos, c) in s.chars().enumerate() {
            let d = (c as usize) - 97;
            max_found_pos[d] = pos;
        }

        let mut j = 0;
        for b in s.as_bytes() {
            let d = (*b as usize) - 97;
            if visited[d] == 1 {
                j += 1;
                continue;
            }

            loop {
                let n = stack.len();
                if n == 0 {
                    break;
                }
                if stack[n - 1] > d && max_found_pos[stack[n - 1]] > j {
                    visited[stack[n - 1]] = 0;
                    stack.pop();
                } else {
                    break;
                }
            }

            visited[d] = 1;
            stack.push(d);
            j += 1;
        }

        for i in 0..stack.len() {
            ans.push((97 + stack[i] as u8) as char);
        }

        ans
    }
}
