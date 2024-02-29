// 1609. Even Odd Tree
// -------------------

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut curr_level = -1;
        let mut last_val = -1;
        let mut queue = VecDeque::new();
        queue.push_back((0, root));

        while queue.len() > 0 {
            let (lvl, node) = queue.pop_front().unwrap();
            let node = node.unwrap();

            if lvl != curr_level {
                curr_level = lvl;
                last_val = if lvl % 2 == 0 { -1 } else { i32::MAX };
            }

            let val = node.borrow().val;
            // even
            if lvl % 2 == 0 {
                if val % 2 == 1 && val > last_val {
                    last_val = val;
                } else {
                    return false;
                }
            // odd
            } else {
                if val % 2 == 0 && val < last_val {
                    last_val = val;
                } else {
                    return false;
                }
            }

            if node.borrow().left.is_some() {
                queue.push_back((lvl + 1, node.borrow().left.clone()));
            }
            if node.borrow().right.is_some() {
                queue.push_back((lvl + 1, node.borrow().right.clone()));
            }
        }

        true
    }
}
