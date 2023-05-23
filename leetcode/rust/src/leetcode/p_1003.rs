// 1003. Check If Word Is Valid After Substitutions

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();

        if s.len() % 3 != 0 {
            return false;
        }

        for c in s.chars() {
            stack.push(c);
            if stack.len() > 2 {
                let l = stack.len() - 3;
                let r = stack.len();
                let p = &stack[l..r].iter().collect::<String>();
                if p == "abc" {
                    stack.drain(l..r);
                }
            }
        }

        if stack.len() == 0 {
            return true;
        }
        false
    }
}
