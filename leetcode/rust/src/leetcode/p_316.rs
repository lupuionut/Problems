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

        s.chars().enumerate().for_each(|(idx, chr)| {
            let chr = (chr as usize) - 97;
            max_found_pos[chr] = idx;
        });

        let mut curr_idx = 0;
        for b in s.as_bytes() {
            let current_letter = (*b as usize) - 97;
            if visited[current_letter] == 1 {
                curr_idx += 1;
                continue;
            }

            loop {
                let n = stack.len();
                if n == 0 {
                    break;
                }
                let last_in_stack = stack[n - 1];
                if last_in_stack > current_letter && max_found_pos[last_in_stack] > curr_idx {
                    visited[last_in_stack] = 0;
                    stack.pop();
                } else {
                    break;
                }
            }

            visited[current_letter] = 1;
            stack.push(current_letter);
            curr_idx += 1;
        }

        for i in 0..stack.len() {
            let chr: char = (97 + stack[i] as u8).into();
            ans.push(chr);
        }

        ans
    }
}
