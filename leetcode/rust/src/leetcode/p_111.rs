// 111. Minimum Depth of Binary Tree
// ---------------------------------

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        let mut queue = VecDeque::new();
        queue.push_back((root, 0));

        while queue.len() > 0 {
            let current = queue.pop_front().unwrap();

            if current.0.is_none() {
                ans = current.1;
                break;
            } else {
                let node = current.0.unwrap();
                let lvl = current.1 + 1;
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    ans = lvl;
                    break;
                }
                if node.borrow().left.is_some() {
                    let left = node.borrow().left.clone();
                    queue.push_back((left, lvl));
                }
                if node.borrow().right.is_some() {
                    let right = node.borrow().right.clone();
                    queue.push_back((right, lvl));
                }
            }
        }

        ans
    }
}
