// 543. Diameter of Binary Tree
// ----------------------------

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut best = 0;

        fn calc(node: Option<Rc<RefCell<TreeNode>>>, mut best: &mut i32) -> i32 {
            let node = node.unwrap();
            let left = if node.borrow().left.is_none() {
                0
            } else {
                1 + calc(node.borrow().left.clone(), best)
            };
            let right = if node.borrow().right.is_none() {
                0
            } else {
                1 + calc(node.borrow().right.clone(), best)
            };
            let curr = *best;
            if (left + right) > curr {
                *best = left + right;
            }
            left.max(right)
        }
        calc(root, &mut best);
        best
    }
}
