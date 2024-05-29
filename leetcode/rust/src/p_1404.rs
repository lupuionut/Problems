// 1404. Number of Steps to Reduce a Number in Binary Representation to One
// ------------------------------------------------------------------------

use std::collections::VecDeque;

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut ans = 0;
        let mut stack = s.chars().rev().collect::<VecDeque<_>>();

        while stack.len() > 1 {
            let i = stack.pop_front().unwrap();
            if i == '0' {
                ans += 1;
            } else {
                ans += 2;
                Solution::add_one(&mut stack);
            }
        }

        if stack[0] == '0' {
            ans += 1;
        }

        ans
    }

    fn add_one(stack: &mut VecDeque<char>) {
        let mut carry = true;
        for i in 0..stack.len() {
            if stack[i] == '0' {
                if carry {
                    stack[i] = '1';
                    carry = false;
                }
            } else {
                if carry {
                    stack[i] = '0';
                }
            }
        }
        if carry {
            stack.push_back('1');
        }
    }
}
