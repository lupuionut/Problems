// 637. Average of Levels in Binary Tree
// -------------------------------------

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut queue = VecDeque::new();
        let mut ans: Vec<f64> = vec![];
        let mut step = 0;
        let mut acc = (0.0, 0.0);

        queue.push_back((root, 0));

        while queue.len() > 0 {
            let (node, lvl) = queue.pop_front().unwrap();
            let node = node.unwrap();
            let node = node.borrow();
            if lvl == step {
                acc.0 += node.val as f64;
                acc.1 += 1.0;
            } else {
                let med = acc.0 / acc.1;
                ans.push(med);
                step += 1;
                acc.0 = node.val as f64;
                acc.1 = 1.0;
            }
            if node.left.is_some() {
                queue.push_back((node.left.clone(), lvl + 1));
            }
            if node.right.is_some() {
                queue.push_back((node.right.clone(), lvl + 1));
            }
        }

        let med = acc.0 / acc.1;
        ans.push(med);

        ans
    }
}
