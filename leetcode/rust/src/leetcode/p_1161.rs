// 1161. Maximum Level Sum of a Binary Tree
// ----------------------------------------

use std::cell::RefCell;
use std::collections::VecDeque;
use std::i32::MIN;
use std::rc::Rc;

impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // (max, level)
        let mut max_found = (MIN, -1);
        let mut current: (i32, i32) = (0, 1);
        let mut queue: VecDeque<(Option<Rc<RefCell<TreeNode>>>, i32)> = VecDeque::new();
        queue.push_back((root, 1));

        while queue.len() > 0 {
            let item = queue.pop_front().unwrap();
            if let Some(n) = item.0 {
                if item.1 == current.1 {
                    current.0 += n.borrow().val;
                } else {
                    if current.0 > max_found.0 {
                        max_found = current;
                    }
                    current = (n.borrow().val, item.1);
                }
                if n.borrow().left.is_some() {
                    queue.push_back((n.borrow().left.clone(), item.1 + 1))
                }
                if n.borrow().right.is_some() {
                    queue.push_back((n.borrow().right.clone(), item.1 + 1))
                }
            }
        }

        if current.0 > max_found.0 {
            max_found = current;
        }
        max_found.1
    }
}
