// 2331. Evaluate Boolean Binary Tree
// ----------------------------------

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn evaluate(node: Option<Rc<RefCell<TreeNode>>>) -> bool {
            let node = node.unwrap();
            let node = node.borrow();
            if node.left.is_none() && node.right.is_none() {
                if node.val == 0 {
                    return false;
                } else {
                    return true;
                }
            }

            if node.val == 2 {
                return evaluate(node.left.clone()) | evaluate(node.right.clone());
            } else {
                return evaluate(node.left.clone()) & evaluate(node.right.clone());
            }
        }
        evaluate(root)
    }
}
