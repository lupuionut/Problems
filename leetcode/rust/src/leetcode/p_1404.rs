// 1404. Number of Steps to Reduce a Number in Binary Representation to One
// ------------------------------------------------------------------------
use std::collections::VecDeque;
impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut ans = 0;
        let mut q = s
            .chars()
            .map(|c| if c == '1' { 1 } else { 0 })
            .collect::<VecDeque<_>>();

        fn add_one(q: &mut VecDeque<i32>) {
            let n = q.len();
            let mut carry = 1;
            for i in (0..n).rev() {
                if q[i] == 1 {
                    if carry == 1 {
                        q[i] = 0;
                    }
                } else {
                    if carry == 1 {
                        q[i] = 1;
                        carry = 0;
                    }
                }
            }
            if carry == 1 {
                q.push_front(1);
            }
        }

        loop {
            if q.len() == 1 {
                if q[0] == 1 {
                    break;
                }
            }
            if let Some(&last) = q.back() {
                if last == 1 {
                    add_one(&mut q);
                } else {
                    q.pop_back();
                }
            }
            ans += 1;
        }
        ans
    }
}
