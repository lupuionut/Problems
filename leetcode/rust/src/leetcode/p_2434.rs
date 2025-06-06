// 2434. Using a Robot to Print the Lexicographically Smallest String
// ------------------------------------------------------------------

use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
    pub fn robot_with_string(s: String) -> String {
        let mut ans: Vec<char> = vec![];
        let mut h: BinaryHeap<(Reverse<usize>, Reverse<usize>)> = BinaryHeap::new();
        let mut stack: Vec<char> = vec![];

        s.chars().enumerate().for_each(|(key, val)| {
            h.push((Reverse(val as usize), Reverse(key)));
        });

        s.chars().enumerate().for_each(|(key, val)| {
            // remove past entries
            while h.len() > 0 {
                if let Some(&(Reverse(chr), Reverse(pos))) = h.peek() {
                    if pos <= key {
                        h.pop();
                    } else {
                        break;
                    }
                }
            }
            // if we are at the end add all entries from stack to answear
            if h.len() == 0 {
                ans.push(val);
                while stack.len() > 0 {
                    ans.push(stack.pop().unwrap());
                }
            } else {
                if let Some(&(Reverse(chr), Reverse(pos))) = h.peek() {
                    // if we have in the future a smaller value, continue to add to stack
                    if chr < val as usize {
                        stack.push(val);
                    } else {
                        // add to answear
                        // take from the stack while the top of stack is smaller or equal than smallest from future
                        ans.push(val);
                        while stack.len() > 0 {
                            let n = stack.len() - 1;
                            if stack[n] as usize <= chr {
                                ans.push(stack[n]);
                                stack.pop();
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        });

        ans.iter().collect()
    }
}
