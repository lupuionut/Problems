// 515. Find Largest Value in Each Tree Row
// ----------------------------------------

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut queue = VecDeque::new();
        let mut level = None;
        queue.push_back((0, root));

        while queue.len() > 0 {
            let (lvl, node) = queue.pop_front().unwrap();
            if let Some(node) = node {
                let node = node.borrow();
                if level.is_none() || lvl != level.unwrap() {
                    ans.push(node.val);
                    level = Some(lvl);
                } else {
                    if node.val > ans[lvl] {
                        ans[lvl] = node.val;
                    }
                }
                if node.left.is_some() {
                    queue.push_back((lvl + 1, node.left.clone()));
                }
                if node.right.is_some() {
                    queue.push_back((lvl + 1, node.right.clone()));
                }
            }
        }
        ans
    }
}
