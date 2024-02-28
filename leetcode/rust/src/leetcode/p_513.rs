// 513. Find Bottom Left Tree Value
// --------------------------------

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut best = (-1, -1);
        let mut queue = VecDeque::new();
        queue.push_back((root, 0));

        while queue.len() > 0 {
            let (node, lvl) = queue.pop_front().unwrap();
            let node = node.unwrap();

            if lvl > best.0 {
                best = (lvl, node.borrow().val);
            }

            if node.borrow().left.is_some() {
                queue.push_back((node.borrow().left.clone(), lvl + 1));
            }
            if node.borrow().right.is_some() {
                queue.push_back((node.borrow().right.clone(), lvl + 1));
            }
        }

        best.1
    }
}
