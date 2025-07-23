// 1717. Maximum Score From Removing Substrings
// --------------------------------------------

impl Solution {
    pub fn maximum_gain(mut s: String, x: i32, y: i32) -> i32 {
        let mut stack = vec![];
        let mut ans = 0;
        s.push('x');
        s.chars().for_each(|c| {
            if stack.len() > 0 {
                let n = stack.len() - 1;
                // ab
                if x > y {
                    if stack[n] == 'a' && c == 'b' {
                        ans += x;
                        stack.pop();
                    } else {
                        stack.push(c);
                    }
                // ba
                } else {
                    if stack[n] == 'b' && c == 'a' {
                        ans += y;
                        stack.pop();
                    } else {
                        stack.push(c);
                    }
                }
            } else {
                stack.push(c);
            }
        });

        let mut nstack = vec![];
        for i in 0..stack.len() {
            if nstack.len() > 0 {
                let n = nstack.len() - 1;
                if nstack[n] == 'a' && stack[i] == 'b' {
                    ans += x;
                    nstack.pop();
                } else if nstack[n] == 'b' && stack[i] == 'a' {
                    ans += y;
                    nstack.pop();
                } else {
                    nstack.push(stack[i]);
                }
            } else {
                nstack.push(stack[i]);
            }
        }
        ans
    }
}
